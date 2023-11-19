use super::{Event, World};
use crate::ecs::{
    component::{Component, ComponentType},
    entity::{Entity, EntityId},
    registry::Registry,
};
use std::marker::PhantomData;

pub trait Builder {
    fn build(&mut self, id: EntityId, world: &World);
}

pub struct ComponentBuilder<T: Component> {
    component: Option<T>,
}

impl<T: Component> ComponentBuilder<T> {
    pub fn new(component: T) -> ComponentBuilder<T> {
        ComponentBuilder {
            component: Some(component),
        }
    }
}

impl<T: Component> Builder for ComponentBuilder<T> {
    fn build(&mut self, id: EntityId, world: &World) {
        let mut registry = world.components_mut::<T>();
        let mut archetypes = world.archetypes.borrow_mut();

        registry.insert(id, self.component.take().unwrap());
        archetypes.add_component::<T>(id);
    }
}

pub struct EntityBuilder {
    id: EntityId,
    builders: Vec<Box<dyn Builder>>,
}

impl EntityBuilder {
    pub fn new() -> EntityBuilder {
        let id = EntityId::zero();

        EntityBuilder {
            id,
            builders: vec![],
        }
    }

    pub fn with<T: Component>(mut self, component: T) -> Self {
        let builder = Box::new(ComponentBuilder::<T>::new(component));
        self.builders.push(builder);

        self
    }
}

impl Event for EntityBuilder {
    fn execute(&mut self, world: &World) {
        {
            let mut entities = world.entities().borrow_mut();
            let mut archetypes = world.archetypes.borrow_mut();

            entities.insert(self.id, Entity::new(self.id));
            archetypes.create_entity(self.id);
        }

        for builder in &mut self.builders {
            builder.build(self.id, world);
        }
    }
}

pub struct DestroyEntity {
    id: EntityId,
}

impl DestroyEntity {
    pub fn new(id: EntityId) -> DestroyEntity {
        DestroyEntity { id }
    }
}

impl Event for DestroyEntity {
    fn execute(&mut self, world: &World) {
        let mut archetypes = world.archetypes.borrow_mut();

        if let Some(archetype) = archetypes.destroy_entity(self.id) {
            let mut entities = world.entities.borrow_mut();
            entities.remove(&self.id);

            for _type in archetype.borrow().types() {
                let id: ComponentType = (*_type).into();
                let resource = world.components_ref(&id);
                resource.borrow_mut().remove(&self.id);
            }
        }
    }
}

pub struct DestroyComponent<T: Component> {
    id: EntityId,
    _marker: PhantomData<T>,
}

impl<T: Component> DestroyComponent<T> {
    pub fn new(id: EntityId) -> DestroyComponent<T> {
        DestroyComponent {
            id,
            _marker: PhantomData,
        }
    }
}

impl<T: Component> Event for DestroyComponent<T> {
    fn execute(&mut self, world: &World) {
        let mut archetypes = world.archetypes.borrow_mut();
        let mut registry = world.components_mut::<T>();

        archetypes.remove_component::<T>(self.id);
        registry.remove(&self.id);
    }
}
