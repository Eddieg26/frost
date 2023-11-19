use super::entity::EntityId;
use std::any::Any;

pub trait Registry {
    fn contains(&self, id: &EntityId) -> bool;
    fn remove(&mut self, id: &EntityId);
    fn clear(&mut self);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
