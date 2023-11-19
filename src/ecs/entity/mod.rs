pub mod registry;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Init,
    Enabled,
    Disabled,
    Destroyed,
}

pub struct Entity {
    id: EntityId,
}

impl Entity {
    pub fn new(id: EntityId) -> Entity {
        Entity { id }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Hash, PartialOrd, Ord)]
pub struct EntityId(pub u64);

impl EntityId {
    pub fn new(id: u64) -> EntityId {
        EntityId(id)
    }

    pub fn zero() -> EntityId {
        EntityId(0)
    }
}

impl Deref for EntityId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EntityId {
    fn deref_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}
