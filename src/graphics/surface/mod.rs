use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window, window::WindowBuilder};

pub struct RenderSurface {
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    adapter: wgpu::Adapter,
    window: Window,
    size: PhysicalSize<u32>,
}

impl RenderSurface {
    pub async fn new(events: &EventLoop<()>) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let window = WindowBuilder::new()
            .with_title("Rust Game Engine")
            .build(events)
            .unwrap();

        let surface =
            unsafe { instance.create_surface(&window) }.expect("Failed to create surface");

        let size = window.inner_size();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        Self {
            surface,
            config,
            adapter,
            window,
            size,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.size.width = width;
        self.size.height = height;
        self.surface.configure(device, &self.config);
    }

    pub fn recreate(&mut self, device: &wgpu::Device) {
        let size = self.window.inner_size();
        self.resize(device, size.width, size.height)
    }

    pub fn configure(&mut self, device: &wgpu::Device) {
        self.surface.configure(device, &self.config);
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn size(&self) -> &PhysicalSize<u32> {
        &self.size
    }
}
