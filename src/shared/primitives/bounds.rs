use crate::shared::Rect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub min: glam::Vec3,
    pub max: glam::Vec3,
}

impl Bounds {
    pub const ZERO: Bounds = Bounds {
        min: glam::Vec3::ZERO,
        max: glam::Vec3::ZERO,
    };

    pub const MAX: Bounds = Bounds {
        min: glam::Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        max: glam::Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
    };

    pub fn new(min: glam::Vec3, max: glam::Vec3) -> Self {
        Self { min, max }
    }

    pub fn from_points(points: &[glam::Vec3]) -> Self {
        let mut min = glam::Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = glam::Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for point in points {
            min = min.min(*point);
            max = max.max(*point);
        }

        Self { min, max }
    }

    pub fn from_bounds(bounds: &[Bounds]) -> Self {
        let mut min = glam::Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = glam::Vec3::new(f32::MIN, f32::MIN, f32::MIN);

        for bound in bounds {
            min = min.min(bound.min);
            max = max.max(bound.max);
        }

        Self { min, max }
    }

    pub fn points(&self) -> [glam::Vec3; 8] {
        [
            self.min,
            glam::Vec3::new(self.min.x, self.min.y, self.max.z),
            glam::Vec3::new(self.min.x, self.max.y, self.min.z),
            glam::Vec3::new(self.min.x, self.max.y, self.max.z),
            glam::Vec3::new(self.max.x, self.min.y, self.min.z),
            glam::Vec3::new(self.max.x, self.min.y, self.max.z),
            glam::Vec3::new(self.max.x, self.max.y, self.min.z),
            self.max,
        ]
    }

    pub fn center(&self) -> glam::Vec3 {
        (self.min + self.max) / 2.0
    }

    pub fn size(&self) -> glam::Vec3 {
        self.max - self.min
    }

    pub fn extents(&self) -> glam::Vec3 {
        self.size() / 2.0
    }

    pub fn contains(&self, point: &glam::Vec3) -> bool {
        self.min.x <= point.x
            && self.min.y <= point.y
            && self.min.z <= point.z
            && self.max.x >= point.x
            && self.max.y >= point.y
            && self.max.z >= point.z
    }

    pub fn intersects(&self, other: &Bounds) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    pub fn intersects_sphere(&self, center: &glam::Vec3, radius: f32) -> bool {
        let mut distance_squared = 0.0;

        if center.x < self.min.x {
            distance_squared += (self.min.x - center.x).powi(2);
        } else if center.x > self.max.x {
            distance_squared += (center.x - self.max.x).powi(2);
        }

        if center.y < self.min.y {
            distance_squared += (self.min.y - center.y).powi(2);
        } else if center.y > self.max.y {
            distance_squared += (center.y - self.max.y).powi(2);
        }

        if center.z < self.min.z {
            distance_squared += (self.min.z - center.z).powi(2);
        } else if center.z > self.max.z {
            distance_squared += (center.z - self.max.z).powi(2);
        }

        distance_squared <= radius.powi(2)
    }

    pub fn transform(&self, transform: &glam::Mat4) -> Self {
        let points = [
            transform.transform_point3(self.min),
            transform.transform_point3(glam::Vec3::new(self.min.x, self.min.y, self.max.z)),
            transform.transform_point3(glam::Vec3::new(self.min.x, self.max.y, self.min.z)),
            transform.transform_point3(glam::Vec3::new(self.min.x, self.max.y, self.max.z)),
            transform.transform_point3(glam::Vec3::new(self.max.x, self.min.y, self.min.z)),
            transform.transform_point3(glam::Vec3::new(self.max.x, self.min.y, self.max.z)),
            transform.transform_point3(glam::Vec3::new(self.max.x, self.max.y, self.min.z)),
            transform.transform_point3(self.max),
        ];

        Self::from_points(&points)
    }
}

impl Into<Rect> for Bounds {
    fn into(self) -> Rect {
        Rect::new(
            self.min.x,
            self.min.y,
            self.max.x - self.min.x,
            self.max.y - self.min.y,
        )
    }
}
