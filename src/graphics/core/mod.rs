pub mod components;
pub mod context;
pub mod gpu;
pub mod primitives;
pub mod resources;
pub mod service;

pub use components::*;
pub use context::*;
pub use gpu::*;
pub use primitives::*;
pub use resources::*;
pub use resources::*;
pub use service::*;

use crate::shared::ResourceId;

pub type BufferId = ResourceId;
pub type TextureId = ResourceId;
pub type MaterialId = ResourceId;
pub type MeshId = ResourceId;
pub type SpriteId = ResourceId;
