use crate::shared::Bounds;

pub struct Sphere {
    pub center: glam::Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: glam::Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn from_points(points: &[glam::Vec3]) -> Self {
        let center = points.iter().sum::<glam::Vec3>() / points.len() as f32;
        let radius = points
            .iter()
            .map(|p| (*p - center).length())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        Self { center, radius }
    }

    pub fn from_spheres(spheres: &[Sphere]) -> Self {
        let center = spheres.iter().map(|s| s.center).sum::<glam::Vec3>() / spheres.len() as f32;
        let radius = spheres
            .iter()
            .map(|s| (s.center - center).length() + s.radius)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        Self { center, radius }
    }

    pub fn contains(&self, point: &glam::Vec3) -> bool {
        (*point - self.center).length() <= self.radius
    }

    pub fn intersects(&self, other: &Sphere) -> bool {
        (self.center - other.center).length() <= self.radius + other.radius
    }

    pub fn intersects_bounds(&self, bounds: &Bounds) -> bool {
        let closest = bounds
            .center()
            .max(self.center.min(bounds.max))
            .min(self.center.max(bounds.min));
        (closest - self.center).length_squared() <= self.radius * self.radius
    }
}
