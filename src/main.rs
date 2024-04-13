use raylib::prelude::*;


#[derive(Clone, Debug)]
struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Rect {
    fn contains(&self, point: &Rect) -> bool {
        point.x >= self.x && point.x <= self.x + self.w &&
        point.y >= self.y && point.y <= self.y + self.h
    }

    fn intersects(&self, range: &Rect) -> bool {
        self.x < range.x + range.w &&
        self.x + self.w > range.x &&
        self.y < range.y + range.h &&
        self.y + self.h > range.y
    }
}

struct QuadTree {
    boundary: Rect,
    capacity: usize,
    points: Vec<Rect>,
    north_west: Option<Box<QuadTree>>,
    north_east: Option<Box<QuadTree>>,
    south_west: Option<Box<QuadTree>>,
    south_east: Option<Box<QuadTree>>,
}

impl QuadTree {
    fn new(boundary: Rect, capacity: usize) -> Self {
        QuadTree {
            boundary,
            capacity,
            points: Vec::new(),
            north_west: None,
            north_east: None,
            south_west: None,
            south_east: None,
        }
    }

    fn insert(&mut self, point: Rect)  -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }

        if self.points.len() < self.capacity && self.north_west.is_none() {
            self.points.push(point);
            return true;
        }

        if self.north_west.is_none() {
            self.subdivide();
        }

        match self.north_west {
            Some(ref mut tree) => {
                if tree.insert(point.clone()) { return true; }
            }
            None => {}
        }

        match self.north_east {
            Some(ref mut tree) => {
                if tree.insert(point.clone()) { return true; }
            }
            None => {}
        }

        match self.south_west {
            Some(ref mut tree) => {
                if tree.insert(point.clone()) { return true; }
            }
            None => {}
        }

        match self.south_east {
            Some(ref mut tree) => {
                if tree.insert(point.clone()) { return true; }
            }
            None => {}
        }

        false
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;

        let nw = Rect { x, y, w: w / 2, h: h / 2 };
        let ne = Rect { x: x + w / 2, y, w: w / 2, h: h / 2 };
        let sw = Rect { x, y: y + h / 2, w: w / 2, h: h / 2 };
        let se = Rect { x: x + w / 2, y: y + h / 2, w: w / 2, h: h / 2 };

        self.north_west = Some(Box::new(QuadTree::new(nw, self.capacity)));
        self.north_east = Some(Box::new(QuadTree::new(ne, self.capacity)));
        self.south_west = Some(Box::new(QuadTree::new(sw, self.capacity)));
        self.south_east = Some(Box::new(QuadTree::new(se, self.capacity)));
    }

    fn query(&self, range: Rect) -> Option<Vec<Rect>> {
        let mut points = Vec::new();

        if !self.boundary.intersects(&range) {
            return None;
        }

        for point in &self.points {
            if range.contains(point) {
                points.push(point.clone());
            }
        }

        if self.north_west.is_none() {
            return Some(points);
        }

        match self.north_west {
            Some(ref tree) => {
                if let Some(mut p) = tree.query(range.clone()) {
                    points.append(&mut p);
                }
            }
            None => {}
        }

        match self.north_east {
            Some(ref tree) => {
                if let Some(mut p) = tree.query(range.clone()) {
                    points.append(&mut p);
                }
            }
            None => {}
        }

        match self.south_west {
            Some(ref tree) => {
                if let Some(mut p) = tree.query(range.clone()) {
                    points.append(&mut p);
                }
            }
            None => {}
        }

        match self.south_east {
            Some(ref tree) => {
                if let Some(mut p) = tree.query(range.clone()) {
                    points.append(&mut p);
                }
            }
            None => {}
        }

        Some(points)
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_lines(self.boundary.x, self.boundary.y, self.boundary.w, self.boundary.h, Color::BLACK);

        if self.north_west.is_some() {
            self.north_west.as_ref().unwrap().draw(d);
        }

        if self.north_east.is_some() {
            self.north_east.as_ref().unwrap().draw(d);
        }

        if self.south_west.is_some() {
            self.south_west.as_ref().unwrap().draw(d);
        }

        if self.south_east.is_some() {
            self.south_east.as_ref().unwrap().draw(d);
        }
    }
}

fn main() {
    let mut rects: Vec<Rect> = Vec::new();
    let mut quadtree = QuadTree::new(Rect { x: 0, y: 0, w: 800, h: 450 }, 4);
    let mut selected_rects: Vec<Rect> = Vec::new();

    let (mut rl, thread) = raylib::init()
    .size(800, 450)
    .title("Quadtree")
    .build();

    rl.set_target_fps(60);

    let mut is_mouse_down = false;
    let mut selection_rect = Rect { x: 0, y: 0, w: 0, h: 0 };

    while !rl.window_should_close() {
        if rl.is_mouse_button_down(
            MouseButton::MOUSE_BUTTON_LEFT
        ) {
            let mouse_pos = rl.get_mouse_position();
            let rect = Rect {
                x: mouse_pos.x as i32,
                y: mouse_pos.y as i32,
                w: 1,
                h: 1,
            };
            rects.push(rect.clone());
            quadtree.insert(rect);
            selection_rect.x = mouse_pos.x as i32;
            selection_rect.y = mouse_pos.y as i32;
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            is_mouse_down = true;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_pos = rl.get_mouse_position();
            selection_rect.x = mouse_pos.x as i32;
            selection_rect.y = mouse_pos.y as i32;
            selection_rect.w = 0;
            selection_rect.h = 0;
        }

        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_RIGHT) {
            is_mouse_down = false;
            selection_rect.w = 0;
            selection_rect.h = 0;
        }

        if is_mouse_down {
            let mouse_pos = rl.get_mouse_position();
            selection_rect.w = mouse_pos.x as i32 - selection_rect.x;
            selection_rect.h = mouse_pos.y as i32 - selection_rect.y;
        }

        let points_in_range = quadtree.query(selection_rect.clone());
        match points_in_range {
            Some(points) => {
                selected_rects = points;
            }
            None => {
                selected_rects.clear();
            }
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        for rect in &rects {
            d.draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::RED);
        }
        for rect in &selected_rects {
            d.draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::BLUE);
        }
        d.draw_rectangle(selection_rect.x, selection_rect.y, selection_rect.w, selection_rect.h, Color::new(0, 255, 0, 100));
        quadtree.draw(&mut d);
    }
}
