use self::events::{entity::EntityBuilder, Event, WorldEvents};
use super::{
    archetype::ArchetypeManager,
    component::{manager::ComponentManager, registry::ComponentRegistry, Component, ComponentType},
    entity::{registry::EntityRegistry, EntityId},
    registry::Registry,
    resource::{manager::ResourceManager, Resource, ResourceType},
};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub mod events;
pub mod query;

type Entities = Rc<RefCell<EntityRegistry>>;
type Archetypes = Rc<RefCell<ArchetypeManager>>;
type Events = Rc<RefCell<WorldEvents>>;

pub struct World {
    entities: Entities,
    archetypes: Archetypes,
    events: Events,
    components: ComponentManager,
    resources: ResourceManager,
}

impl World {
    pub fn new(components: ComponentManager, resources: ResourceManager) -> World {
        let entities = Rc::new(RefCell::new(EntityRegistry::new()));
        let archetypes = Rc::new(RefCell::new(ArchetypeManager::new()));
        let events = Rc::new(RefCell::new(WorldEvents::new()));

        World {
            components,
            resources,
            entities,
            archetypes,
            events,
        }
    }

    pub fn components<T: Component>(&self) -> Ref<'_, ComponentRegistry<T>> {
        self.components.registry::<T>()
    }

    pub fn components_mut<T: Component>(&self) -> RefMut<'_, ComponentRegistry<T>> {
        self.components.registry_mut::<T>()
    }

    pub fn components_ref(&self, type_id: &ComponentType) -> &Rc<RefCell<Box<dyn Registry>>> {
        self.components.registry_ref(type_id)
    }

    pub fn resource<T: Resource>(&self) -> Ref<'_, T> {
        self.resources.resource::<T>()
    }

    pub fn resource_mut<T: Resource>(&self) -> RefMut<'_, T> {
        self.resources.resource_mut::<T>()
    }

    pub fn resource_ref(&self, type_id: &ResourceType) -> &Rc<RefCell<Box<dyn Resource>>> {
        self.resources.resource_ref(type_id)
    }

    pub fn entities(&self) -> &Rc<RefCell<EntityRegistry>> {
        &self.entities
    }

    pub fn archetypes(&self) -> &Rc<RefCell<ArchetypeManager>> {
        &self.archetypes
    }
}

impl World {
    pub fn add_event(&self, event: impl Event) {
        let mut events = self.events.borrow_mut();
        events.add(event);
    }

    pub fn spawn(&self, _builder: EntityBuilder) {
        todo!()
    }

    pub fn spawn_empty(&self) -> EntityId {
        todo!()
    }

    pub fn destroy(&self, _id: &EntityId) {
        todo!()
    }

    pub fn remove<T: Component>(&self, _id: &EntityId) {
        todo!()
    }
}
