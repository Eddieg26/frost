use super::color::Color;
use crate::graphics::TextureId;

#[derive(Clone, Copy, Debug)]
pub enum ProjectionMode {
    Perspective,
    Orthographic,
}

pub enum RenderTarget {
    Window,
    Texture(TextureId),
}

pub struct RenderView {
    view: glam::Mat4,
    size: f32,
    near: f32,
    far: f32,
    fov: f32,
    order: usize,
    mode: ProjectionMode,
    clear_color: Color,
    target: RenderTarget,
}

impl RenderView {
    pub fn new(
        view: glam::Mat4,
        size: f32,
        near: f32,
        far: f32,
        fov: f32,
        order: usize,
        mode: ProjectionMode,
        clear_color: Color,
        target: RenderTarget,
    ) -> Self {
        Self {
            view,
            size,
            near,
            far,
            fov,
            order,
            mode,
            clear_color,
            target,
        }
    }

    pub fn view(&self) -> &glam::Mat4 {
        &self.view
    }

    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn mode(&self) -> ProjectionMode {
        self.mode
    }

    pub fn clear_color(&self) -> Color {
        self.clear_color
    }

    pub fn target(&self) -> &RenderTarget {
        &self.target
    }

    pub fn orthographic(&self, aspect: f32) -> glam::Mat4 {
        let size = self.size;
        let near = self.near;
        let far = self.far;

        let left = -size * aspect;
        let right = size * aspect;
        let bottom = -size;
        let top = size;

        glam::Mat4::orthographic_rh(left, right, bottom, top, near, far)
    }

    pub fn perspective(&self, aspect: f32) -> glam::Mat4 {
        let fov = self.fov;
        let near = self.near;
        let far = self.far;

        glam::Mat4::perspective_rh(fov, aspect, near, far)
    }
}
