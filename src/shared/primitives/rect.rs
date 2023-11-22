#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const ZERO: Rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 0.0,
        height: 0.0,
    };

    pub const MAX: Rect = Rect {
        x: std::f32::MIN,
        y: std::f32::MIN,
        width: std::f32::MAX,
        height: std::f32::MAX,
    };

    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn center(&self) -> glam::Vec2 {
        glam::Vec2::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn points(&self) -> [glam::Vec2; 4] {
        [
            glam::Vec2::new(self.x, self.y),
            glam::Vec2::new(self.x + self.width, self.y),
            glam::Vec2::new(self.x, self.y + self.height),
            glam::Vec2::new(self.x + self.width, self.y + self.height),
        ]
    }

    pub fn transform(&self, transform: &glam::Mat4) -> Rect {
        let points = self.points();
        let mut min_x = std::f32::MAX;
        let mut min_y = std::f32::MAX;
        let mut max_x = std::f32::MIN;
        let mut max_y = std::f32::MIN;
        for point in points.iter() {
            let point = transform.transform_point3(glam::Vec3::new(point.x, point.y, 0.0));
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
}
