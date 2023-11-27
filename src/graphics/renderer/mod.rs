use self::{
    node::RenderPassKind,
    pass::{RenderPass, RenderPassBuilder},
};
use super::{gpu::Gpu, surface::RenderSurface, Graphics, TextureId};
use std::{collections::HashMap, rc::Rc};

pub mod node;
pub mod pass;
pub mod subpass;

pub struct TextureInfo {
    pub format: wgpu::TextureFormat,
    pub usage: wgpu::TextureUsages,
}

pub struct Renderer {
    gpu: Rc<Gpu>,
    textures: HashMap<TextureId, wgpu::TextureView>,
    texture_info: HashMap<TextureId, TextureInfo>,
    passes: Vec<RenderPass>,
}

impl Renderer {
    pub fn new(
        gpu: Rc<Gpu>,
        textures: HashMap<TextureId, wgpu::TextureView>,
        texture_info: HashMap<TextureId, TextureInfo>,
        passes: Vec<RenderPass>,
    ) -> Self {
        Self {
            gpu,
            textures,
            passes,
            texture_info,
        }
    }

    pub fn render(
        &self,
        graphics: &mut Graphics,
        surface: &RenderSurface,
    ) -> Result<(), wgpu::SurfaceError> {
        let mut encoder =
            self.gpu
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        let surface_texture = surface.surface().get_current_texture()?;
        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.gpu.queue().submit(std::iter::once(encoder.finish()));

        surface_texture.present();

        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        for (id, info) in &self.texture_info {
            let texture = self.gpu.device().create_texture(&wgpu::TextureDescriptor {
                label: Some("Texture"),
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: info.format,
                usage: info.usage,
                view_formats: &vec![],
            });

            let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

            self.textures.insert(id.clone(), view);
        }
    }
}

pub struct RendererBuilder {
    textures: HashMap<TextureId, TextureInfo>,
}

impl RendererBuilder {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, id: TextureId, info: TextureInfo) {
        self.textures.insert(id, info);
    }

    pub fn build(mut self, gpu: &Rc<Gpu>, surface: &RenderSurface) -> Renderer {
        let passes = self.create_passes();

        let passes = passes.into_iter().map(|p| p.build(gpu)).collect::<Vec<_>>();

        let size = surface.window().inner_size();

        let textures = self
            .textures
            .iter()
            .map(|(id, info)| {
                let texture = gpu.device().create_texture(&wgpu::TextureDescriptor {
                    label: Some("Texture"),
                    size: wgpu::Extent3d {
                        width: size.width,
                        height: size.height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: info.format,
                    usage: info.usage,
                    view_formats: &vec![],
                });

                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

                (id.clone(), view)
            })
            .collect::<HashMap<_, _>>();

        Renderer::new(gpu.clone(), textures, self.textures, passes)
    }

    fn create_passes(&mut self) -> Vec<RenderPassBuilder> {
        let forward = RenderPassBuilder::new(RenderPassKind::Forward);

        vec![forward]
    }
}
