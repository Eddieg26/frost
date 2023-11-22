use crate::shared::{Bounds, Rect, Sphere};

use super::plane::Plane;

pub struct Frustum {
    pub planes: [Plane; 6],
    pub axes: [glam::Vec3; 3],
    pub half_extents: glam::Vec3,
}

impl Frustum {
    pub fn new(
        position: glam::Vec3,
        rotation: glam::Quat,
        aspect: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Frustum {
        let mut planes = [Plane::ZERO; 6];

        let tan_fov = (fov * 0.5).tan();
        let near_height = tan_fov * near;
        let near_width = near_height * aspect;
        let far_height = tan_fov * far;
        let far_width = far_height * aspect;

        let forward = rotation * glam::Vec3::Z;
        let right = rotation * glam::Vec3::X;
        let up = rotation * glam::Vec3::Y;

        let near_center = position + forward * near;
        let far_center = position + forward * far;

        let near_top_left = near_center + up * near_height - right * near_width;
        let near_top_right = near_center + up * near_height + right * near_width;
        let near_bottom_left = near_center - up * near_height - right * near_width;
        let near_bottom_right = near_center - up * near_height + right * near_width;

        let far_top_left = far_center + up * far_height - right * far_width;
        let far_top_right = far_center + up * far_height + right * far_width;
        let far_bottom_left = far_center - up * far_height - right * far_width;
        let far_bottom_right = far_center - up * far_height + right * far_width;

        planes[0] =
            Plane::from_points(&[near_top_right, near_top_left, far_top_left, far_top_right]);
        planes[1] = Plane::from_points(&[
            near_bottom_left,
            near_bottom_right,
            far_bottom_right,
            far_bottom_left,
        ]);
        planes[2] = Plane::from_points(&[
            near_top_left,
            near_bottom_left,
            far_bottom_left,
            far_top_left,
        ]);
        planes[3] = Plane::from_points(&[
            near_bottom_right,
            near_top_right,
            far_top_right,
            far_bottom_right,
        ]);
        planes[4] = Plane::from_points(&[
            near_top_left,
            near_top_right,
            near_bottom_right,
            near_bottom_left,
        ]);
        planes[5] = Plane::from_points(&[
            far_top_right,
            far_top_left,
            far_bottom_left,
            far_bottom_right,
        ]);

        let axes = [right, up, forward];

        let half_extents = glam::Vec3::new(far_width, far_height, far_width);

        Frustum {
            planes,
            axes,
            half_extents,
        }
    }

    pub fn contains_point(&self, point: glam::Vec3) -> bool {
        for plane in self.planes.iter() {
            if plane.distance_to_point(point) < 0.0 {
                return false;
            }
        }

        true
    }

    pub fn contains_sphere(&self, sphere: &Sphere) -> bool {
        for plane in self.planes.iter() {
            if plane.distance_to_sphere(sphere.center, sphere.radius) < 0.0 {
                return false;
            }
        }

        true
    }

    pub fn contains_bounds(&self, bounds: &Bounds) -> bool {
        let points = bounds.points();

        for plane in self.planes.iter() {
            let mut inside = false;

            for point in points.iter() {
                if plane.distance_to_point(*point) >= 0.0 {
                    inside = true;
                    break;
                }
            }

            if !inside {
                return false;
            }
        }

        true
    }

    pub fn intersects_bounds(&self, bounds: &Bounds) -> bool {
        let points = bounds.points();

        for plane in self.planes.iter() {
            let mut inside = false;

            for point in points.iter() {
                if plane.distance_to_point(*point) >= 0.0 {
                    inside = true;
                    break;
                }
            }

            if !inside {
                return false;
            }
        }

        true
    }

    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        for plane in self.planes.iter() {
            if plane.distance_to_sphere(sphere.center, sphere.radius) < 0.0 {
                return false;
            }
        }

        true
    }

    pub fn rect(&self) -> Rect {
        let mut min = glam::Vec3::ZERO;
        let mut max = glam::Vec3::ZERO;

        for plane in self.planes.iter() {
            if plane.normal.x > 0.0 {
                max.x += plane.normal.x * self.half_extents.x;
            } else {
                min.x += plane.normal.x * self.half_extents.x;
            }

            if plane.normal.y > 0.0 {
                max.y += plane.normal.y * self.half_extents.y;
            } else {
                min.y += plane.normal.y * self.half_extents.y;
            }

            if plane.normal.z > 0.0 {
                max.z += plane.normal.z * self.half_extents.z;
            } else {
                min.z += plane.normal.z * self.half_extents.z;
            }
        }

        Rect::new(min.x, min.y, max.x - min.x, max.y - min.y)
    }
}
