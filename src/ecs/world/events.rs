use crate::ecs::{observer::EntityEvent, Component, Entity, EntityId, Registry};

pub struct AddComponent<T: Component> {
    entity_id: EntityId,
    component: Option<T>,
}

impl<T: Component> AddComponent<T> {
    pub fn new(entity_id: EntityId, component: T) -> Self {
        Self {
            entity_id,
            component: Some(component),
        }
    }
}

impl<T: Component> EntityEvent for AddComponent<T> {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world
            .components_mut::<T>()
            .insert(self.entity_id, self.component.take().unwrap());
        world.archetypes_mut().add_component::<T>(self.entity_id);
        self.entity_id
    }
}

pub struct CreateEntity {
    entity_id: EntityId,
    components: Vec<Box<dyn EntityEvent>>,
}

impl CreateEntity {
    pub fn new() -> Self {
        Self {
            entity_id: EntityId::uuid(),
            components: Vec::new(),
        }
    }

    pub fn id(&self) -> &EntityId {
        &self.entity_id
    }

    pub fn with<T: Component>(mut self, component: T) -> Self {
        self.components
            .push(Box::new(AddComponent::new(self.entity_id, component)));
        self
    }
}

impl EntityEvent for CreateEntity {
    fn execute(&mut self, world: &super::World) -> EntityId {
        for component in self.components.iter_mut() {
            component.execute(world);
        }

        world
            .entities
            .borrow_mut()
            .insert(self.entity_id, Entity::new(self.entity_id));

        self.entity_id
    }
}

pub struct EnableComponent<T: Component> {
    entity_id: EntityId,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component> EnableComponent<T> {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Component> EntityEvent for EnableComponent<T> {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world.components_mut::<T>().enable(&self.entity_id);
        self.entity_id
    }
}

pub struct DisableComponent<T: Component> {
    entity_id: EntityId,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component> DisableComponent<T> {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Component> EntityEvent for DisableComponent<T> {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world.components_mut::<T>().disable(&self.entity_id);
        self.entity_id
    }
}

pub struct EnableEntity {
    entity_id: EntityId,
}

impl EnableEntity {
    pub fn new(entity_id: EntityId) -> Self {
        Self { entity_id }
    }
}

impl EntityEvent for EnableEntity {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world.entities_mut().enable(&self.entity_id);
        self.entity_id
    }
}

pub struct DisableEntity {
    entity_id: EntityId,
}

impl DisableEntity {
    pub fn new(entity_id: EntityId) -> Self {
        Self { entity_id }
    }
}

impl EntityEvent for DisableEntity {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world.entities_mut().disable(&self.entity_id);
        self.entity_id
    }
}

pub struct DestroyEntity {
    entity_id: EntityId,
}

impl DestroyEntity {
    pub fn new(entity_id: EntityId) -> Self {
        Self { entity_id }
    }
}

impl EntityEvent for DestroyEntity {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world.entities_mut().destroy(&self.entity_id);
        world.component_manager().destroy(&self.entity_id);
        world.archetypes_mut().destroy_entity(self.entity_id);
        self.entity_id
    }
}

pub struct RemoveComponent<T: Component> {
    entity_id: EntityId,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component> RemoveComponent<T> {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Component> EntityEvent for RemoveComponent<T> {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world.components_mut::<T>().destroy(&self.entity_id);
        world.archetypes_mut().remove_component::<T>(self.entity_id);
        self.entity_id
    }
}

pub struct UpdateComponent<T: Component> {
    entity_id: EntityId,
    component: Option<T>,
}

impl<T: Component> UpdateComponent<T> {
    pub fn new(entity_id: EntityId, component: T) -> Self {
        Self {
            entity_id,
            component: Some(component),
        }
    }
}

impl<T: Component> EntityEvent for UpdateComponent<T> {
    fn execute(&mut self, world: &super::World) -> EntityId {
        world
            .components_mut::<T>()
            .insert(self.entity_id, self.component.take().unwrap());
        self.entity_id
    }
}
