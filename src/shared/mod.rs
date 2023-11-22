use std::{
    any::TypeId,
    collections::hash_map::DefaultHasher,
    fmt::Display,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    path::Path,
};

pub mod ext;
pub mod primitives;

pub use primitives::*;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct ResourceId(u64);

impl Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl ResourceId {
    pub fn new(name: &str) -> ResourceId {
        let mut hasher = DefaultHasher::new();

        name.hash(&mut hasher);

        ResourceId(hasher.finish())
    }

    pub fn zero() -> ResourceId {
        ResourceId(0)
    }
}

impl From<&Path> for ResourceId {
    fn from(value: &Path) -> Self {
        ResourceId::new(value.to_str().unwrap())
    }
}

impl From<&str> for ResourceId {
    fn from(value: &str) -> Self {
        ResourceId::new(value)
    }
}

impl From<String> for ResourceId {
    fn from(value: String) -> Self {
        ResourceId::new(&value)
    }
}

impl Deref for ResourceId {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ResourceId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
pub struct ResourceType(u64);

impl ResourceType {
    pub fn new(type_id: u64) -> ResourceType {
        ResourceType(type_id)
    }
}

impl From<TypeId> for ResourceType {
    fn from(value: TypeId) -> Self {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        ResourceType(hasher.finish())
    }
}
