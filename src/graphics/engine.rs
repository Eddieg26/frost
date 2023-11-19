use std::rc::Rc;
use winit::window::Window;

use super::core::GpuDevice;

pub struct GraphicsEngine {
    device: Rc<GpuDevice>,
}

impl GraphicsEngine {
    pub(crate) async fn new(window: Window) -> GraphicsEngine {
        let device = GpuDevice::new(window).await;

        GraphicsEngine {
            device: Rc::new(device),
        }
    }

    pub fn device(&self) -> Rc<GpuDevice> {
        self.device.clone()
    }
}
