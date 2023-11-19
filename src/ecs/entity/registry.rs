use super::{Entity, EntityId};
use crate::ecs::registry::Registry;
use std::collections::HashMap;

pub struct EntityRegistry {
    entities: HashMap<EntityId, Entity>,
}

impl EntityRegistry {
    pub fn new() -> EntityRegistry {
        EntityRegistry {
            entities: HashMap::new(),
        }
    }

    pub fn get(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub fn get_mut(&mut self, id: &EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(id)
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
