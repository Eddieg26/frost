use super::{
    device::GpuDevice,
    renderer::{Renderer, RendererBuilder},
    surface::RenderSurface,
    Graphics,
};
use std::rc::Rc;
use winit::event_loop::EventLoop;

pub struct GraphicsEngine {
    device: Rc<GpuDevice>,
    surface: RenderSurface,
    renderer: Renderer,
}

impl GraphicsEngine {
    pub(crate) async fn new(events: &EventLoop<()>) -> GraphicsEngine {
        let surface = RenderSurface::new(events).await;
        let device = Rc::new(GpuDevice::new(surface.adapter()).await);
        let renderer = RendererBuilder::new().build(&device, &surface);

        GraphicsEngine {
            device,
            surface,
            renderer,
        }
    }

    pub fn device(&self) -> &Rc<GpuDevice> {
        &self.device
    }

    pub fn surface(&self) -> &RenderSurface {
        &self.surface
    }

    pub fn window(&self) -> &winit::window::Window {
        self.surface.window()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.resize(self.device.device(), width, height);
        self.renderer.resize(width, height);
    }

    pub fn render(&mut self, graphics: &mut Graphics) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(graphics, &self.surface)
    }
}
