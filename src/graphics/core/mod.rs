pub mod color;
pub mod device;
pub mod material;
pub mod mesh;
pub mod texture;
pub mod vertex;

pub use color::*;
pub use device::*;
pub use material::*;
pub use mesh::*;
pub use texture::*;
pub use vertex::*;

use crate::shared::ResourceId;

pub type BufferId = ResourceId;
pub type TextureId = ResourceId;
pub type MaterialId = ResourceId;
pub type MeshId = ResourceId;
