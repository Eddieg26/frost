use crate::{
    ecs::Component,
    graphics::{color::Color, frustum::Frustum, view::ProjectionMode},
};

pub struct Camera {
    pub size: f32,
    pub near: f32,
    pub far: f32,
    pub fov: f32,
    pub order: usize,
    pub mode: ProjectionMode,
    pub clear_color: Color,
}

impl Camera {
    pub fn new(
        size: f32,
        near: f32,
        far: f32,
        fov: f32,
        order: usize,
        mode: ProjectionMode,
        clear_color: Color,
    ) -> Self {
        Self {
            size,
            near,
            far,
            fov,
            order,
            mode,
            clear_color,
        }
    }

    pub fn orthographic(&self, aspect: f32) -> glam::Mat4 {
        let Self {
            size, near, far, ..
        } = self;

        let left = -size * aspect;
        let right = size * aspect;
        let bottom = -size;
        let top = size;

        glam::Mat4::orthographic_rh(left, right, bottom, *top, *near, *far)
    }

    pub fn perspective(&self, aspect: f32) -> glam::Mat4 {
        let Self { fov, near, far, .. } = self;
        glam::Mat4::perspective_rh(*fov, aspect, *near, *far)
    }

    pub fn frustum(&self, view: glam::Mat4, aspect: f32) -> Frustum {
        let (_, rotation, position) = view.to_scale_rotation_translation();
        Frustum::new(position, rotation, aspect, self.fov, self.near, self.far)
    }
}

impl Component for Camera {}
