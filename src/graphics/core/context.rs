use super::view::RenderView;
use crate::graphics::{Graphics, TextureId};
use std::collections::HashMap;

pub struct RenderContext<'a> {
    graphics: &'a Graphics,
    view: &'a RenderView,
    textures: &'a HashMap<TextureId, wgpu::TextureView>,
}

impl<'a> RenderContext<'a> {
    pub fn new(
        graphics: &'a Graphics,
        view: &'a RenderView,
        textures: &'a HashMap<TextureId, wgpu::TextureView>,
    ) -> Self {
        Self {
            graphics,
            view,
            textures,
        }
    }

    pub fn graphics(&self) -> &'a Graphics {
        self.graphics
    }

    pub fn view(&self) -> &'a RenderView {
        self.view
    }

    pub fn texture(&self, id: TextureId) -> &'a wgpu::TextureView {
        self.textures.get(&id).unwrap()
    }
}
