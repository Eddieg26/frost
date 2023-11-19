pub mod manager;
use std::any::Any;

pub use crate::shared::{ResourceId, ResourceType};

pub trait Resource: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
