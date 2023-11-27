use super::{Entity, EntityId};
use crate::ecs::registry::Registry;
use std::collections::{HashMap, HashSet};

pub struct EntityRegistry {
    entities: HashMap<EntityId, Entity>,
    enabled: HashSet<EntityId>,
    disabled: HashSet<EntityId>,
    destroyed: HashSet<EntityId>,
}

impl EntityRegistry {
    pub fn new() -> EntityRegistry {
        EntityRegistry {
            entities: HashMap::new(),
            destroyed: HashSet::new(),
            disabled: HashSet::new(),
            enabled: HashSet::new(),
        }
    }

    pub fn get(&self, id: &EntityId) -> Option<&Entity> {
        if self.destroyed.contains(id) || self.disabled.contains(id) {
            None
        } else {
            self.entities.get(id)
        }
    }

    pub fn get_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        if self.destroyed.contains(id) || self.disabled.contains(id) {
            None
        } else {
            self.entities.get_mut(id)
        }
    }

    pub fn enabled(&self) -> impl Iterator<Item = &EntityId> {
        self.enabled.iter()
    }

    pub fn disabled(&self) -> impl Iterator<Item = &EntityId> {
        self.disabled.iter()
    }

    pub fn destroyed(&self) -> impl Iterator<Item = &EntityId> {
        self.destroyed.iter()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&EntityId, &Entity)> {
        self.entities.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&EntityId, &mut Entity)> {
        self.entities.iter_mut()
    }

    pub fn extend(&mut self, iter: impl Iterator<Item = (EntityId, Entity)>) {
        self.entities.extend(iter)
    }

    pub fn create(&mut self) -> &Entity {
        let id = EntityId::zero();
        let entity = Entity::new(id);
        self.entities.insert(id, entity);

        self.entities.get(&id).unwrap()
    }

    pub fn insert(&mut self, id: EntityId, entity: Entity) {
        self.entities.insert(id, entity);
    }
}

impl Registry for EntityRegistry {
    fn contains(&self, id: &EntityId) -> bool {
        self.entities.contains_key(id)
    }

    fn remove(&mut self, id: &EntityId) {
        self.entities.remove(id);
    }

    fn clear(&mut self) {
        self.entities.clear()
    }

    fn update(&mut self) {
        self.entities.retain(|id, _| !self.destroyed.contains(id));
        self.destroyed.clear();
    }

    fn enable(&mut self, id: &EntityId) {
        if !self.destroyed.contains(id) {
            self.enabled.insert(*id);
            self.disabled.remove(id);
        }
    }

    fn disable(&mut self, id: &EntityId) {
        if !self.destroyed.contains(id) {
            self.disabled.insert(*id);
            self.enabled.remove(id);
        }
    }

    fn destroy(&mut self, id: &EntityId) {
        if !self.destroyed.contains(id) {
            self.destroyed.insert(*id);
            self.enabled.remove(id);
            self.disabled.remove(id);
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
