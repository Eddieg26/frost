use super::{
    gpu::Gpu,
    renderer::{Renderer, RendererBuilder},
    surface::RenderSurface,
    Config, Graphics,
};
use std::rc::Rc;
use winit::event_loop::EventLoop;

pub struct GraphicsEngine {
    gpu: Rc<Gpu>,
    surface: RenderSurface,
    renderer: Renderer,
}

impl GraphicsEngine {
    pub(crate) async fn new(events: &EventLoop<()>) -> GraphicsEngine {
        let surface = RenderSurface::new(events).await;
        let gpu = Rc::new(Gpu::new(surface.adapter()).await);
        let renderer = RendererBuilder::new().build(&gpu, &surface);

        GraphicsEngine {
            gpu,
            surface,
            renderer,
        }
    }

    pub fn gpu(&self) -> &Rc<Gpu> {
        &self.gpu
    }

    pub fn surface(&self) -> &RenderSurface {
        &self.surface
    }

    pub fn config(&self) -> Config {
        Config::new(self.surface.format(), self.surface.depth_format())
    }

    pub fn window(&self) -> &winit::window::Window {
        self.surface.window()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize(self.gpu.device(), width, height);
        self.renderer.resize(width, height);
    }

    pub fn render(&mut self, graphics: &mut Graphics) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(graphics, &self.surface)
    }
}
