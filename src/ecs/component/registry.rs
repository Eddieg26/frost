use super::Component;
use crate::ecs::{entity::EntityId, registry::Registry};
use std::{any::Any, collections::HashMap};

pub struct ComponentRegistry<T: Component> {
    register: HashMap<EntityId, T>,
}

impl<T: Component> ComponentRegistry<T> {
    pub fn new() -> ComponentRegistry<T> {
        ComponentRegistry {
            register: HashMap::new(),
        }
    }

    pub fn get(&self, id: &EntityId) -> Option<&T> {
        self.register.get(id)
    }

    pub fn get_mut(&mut self, id: &EntityId) -> Option<&mut T> {
        self.register.get_mut(id)
    }

    pub fn all<'a>(&'a self) -> impl Iterator<Item = (&EntityId, &T)> {
        self.register.iter()
    }

    pub fn all_mut<'a>(&'a mut self) -> impl Iterator<Item = (&EntityId, &mut T)> {
        self.register.iter_mut()
    }

    pub fn extend<'a>(&'a mut self, iter: impl Iterator<Item = (EntityId, T)>) {
        self.register.extend(iter)
    }

    pub fn insert(&mut self, id: EntityId, data: T) {
        self.register.insert(id, data);
    }
}

impl<T: Component> ComponentRegistry<T> {
    pub fn iter<'a>(&'a self, entities: &'a Vec<EntityId>) -> impl Iterator<Item = (EntityId, &T)> {
        self.register.iter().filter_map(|(id, component)| {
            if entities.contains(id) {
                Some((*id, component))
            } else {
                None
            }
        })
    }

    pub fn iter_mut<'a>(
        &'a mut self,
        entities: &'a Vec<EntityId>,
    ) -> impl Iterator<Item = (EntityId, &mut T)> {
        self.register.iter_mut().filter_map(|(id, component)| {
            if entities.contains(id) {
                Some((*id, component))
            } else {
                None
            }
        })
    }
}

impl<T: Component> Registry for ComponentRegistry<T> {
    fn contains(&self, id: &EntityId) -> bool {
        self.register.contains_key(id)
    }

    fn remove(&mut self, id: &EntityId) {
        self.register.remove(id);
    }

    fn clear(&mut self) {
        self.register.clear()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
