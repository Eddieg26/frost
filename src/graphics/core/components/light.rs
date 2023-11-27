use crate::{
    ecs::Component,
    graphics::color::Color,
    shared::{Bounds, Rect},
    spatial::{octtree::Object3D, quadtree::Object2D},
};

#[derive(Clone, Copy)]
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

    pub fn rect(&self, transform: &glam::Mat4) -> Rect {
        match self.kind {
            LightKind::Directional => Rect::MAX,
            LightKind::Point | LightKind::Spot => {
                let min = transform.transform_point3(glam::Vec3::new(
                    -self.range,
                    -self.range,
                    -self.range,
                ));
                let max =
                    transform.transform_point3(glam::Vec3::new(self.range, self.range, self.range));

                Rect::from_extents(min.truncate(), max.truncate())
            }
        }
    }

    pub fn bounds(&self, transform: &glam::Mat4) -> Bounds {
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

pub enum LightRef {
    D2(Light2D),
    D3(Light3D),
}

pub struct Light2D {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub kind: LightKind,
    pub spot_angle: f32,
    pub transform: glam::Mat4,
    pub rect: Rect,
}

impl Light2D {
    pub fn new(light: &Light, transform: glam::Mat4) -> Self {
        let rect = light.rect(&transform);

        Self {
            color: light.color,
            intensity: light.intensity,
            range: light.range,
            kind: light.kind,
            spot_angle: light.spot_angle,
            transform,
            rect,
        }
    }
}

impl Object2D for Light2D {
    fn rect(&self) -> &Rect {
        &self.rect
    }
}

pub struct Light3D {
    pub color: Color,
    pub intensity: f32,
    pub range: f32,
    pub kind: LightKind,
    pub spot_angle: f32,
    pub transform: glam::Mat4,
    pub bounds: Bounds,
}

impl Light3D {
    pub fn new(light: &Light, transform: glam::Mat4) -> Self {
        let bounds = light.bounds(&transform);

        Self {
            color: light.color,
            intensity: light.intensity,
            range: light.range,
            kind: light.kind,
            spot_angle: light.spot_angle,
            transform,
            bounds,
        }
    }
}

impl Object3D for Light3D {
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}
