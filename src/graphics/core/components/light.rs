use crate::{
    ecs::Component,
    graphics::color::Color,
    shared::{Bounds, Rect},
};

pub enum LightKind {
    Directional,
    Point,
    Spot,
}

pub struct Light {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub kind: LightKind,
    pub spot_angle: f32,
}

impl Light {
    pub fn new(color: Color, intensity: f32, range: f32, kind: LightKind, spot_angle: f32) -> Self {
        Self {
            color,
            intensity,
            range,
            kind,
            spot_angle,
        }
    }

    pub fn rect(&self) -> Rect {
        match self.kind {
            LightKind::Directional => Rect::MAX,
            LightKind::Point | LightKind::Spot => {
                Rect::new(-self.range, -self.range, self.range * 2.0, self.range * 2.0)
            }
        }
    }

    pub fn bounds(&self, transform: glam::Mat4) -> Bounds {
        match self.kind {
            LightKind::Directional => Bounds::MAX,
            LightKind::Point | LightKind::Spot => {
                let min = transform.transform_point3(glam::Vec3::new(
                    -self.range,
                    -self.range,
                    -self.range,
                ));
                let max =
                    transform.transform_point3(glam::Vec3::new(self.range, self.range, self.range));

                Bounds::new(min, max)
            }
        }
    }
}

impl Component for Light {}
