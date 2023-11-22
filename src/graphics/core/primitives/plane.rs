pub struct Plane {
    pub normal: glam::Vec3,
    pub distance: f32,
}

impl Plane {
    pub const ZERO: Self = Self {
        normal: glam::Vec3::ZERO,
        distance: 0.0,
    };

    pub fn new(normal: glam::Vec3, distance: f32) -> Self {
        Self { normal, distance }
    }

    pub fn from_points(points: &[glam::Vec3]) -> Self {
        let normal = (points[1] - points[0])
            .cross(points[2] - points[0])
            .normalize();
        let distance = normal.dot(points[0]);

        Self { normal, distance }
    }

    pub fn from_point_and_normal(point: glam::Vec3, normal: glam::Vec3) -> Self {
        let distance = normal.dot(point);

        Self { normal, distance }
    }

    pub fn from_point_and_direction(point: glam::Vec3, direction: glam::Vec3) -> Self {
        let normal = direction.normalize();
        let distance = normal.dot(point);

        Self { normal, distance }
    }

    pub fn distance_to_point(&self, point: glam::Vec3) -> f32 {
        self.normal.dot(point) - self.distance
    }

    pub fn distance_to_sphere(&self, center: glam::Vec3, radius: f32) -> f32 {
        self.distance_to_point(center) - radius
    }
}
