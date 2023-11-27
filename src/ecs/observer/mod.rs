use super::{EntityId, Resource, World};
use std::{any::TypeId, collections::HashMap};

pub trait Observer: 'static {
    fn observe(&self, entities: &[EntityId], world: &World);
}

impl<T: Fn(&[EntityId], &World) + 'static> Observer for T {
    fn observe(&self, entities: &[EntityId], world: &World) {
        self(entities, world)
    }
}

pub trait EntityEvent: 'static {
    fn execute(&mut self, world: &World) -> EntityId;
}

pub struct EventManager {
    events: HashMap<TypeId, Vec<Box<dyn EntityEvent>>>,
    observers: Observers,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager {
            events: HashMap::new(),
            observers: Observers::new(),
        }
    }

    pub fn register<T: EntityEvent>(&mut self, event: T) {
        let type_id = std::any::TypeId::of::<T>();

        self.events
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(event));
    }

    pub fn observe<T: EntityEvent>(&mut self, system: impl Observer) {
        self.observers.observe::<T>(system);
    }

    pub fn flush(&mut self, world: &World) {
        for (type_id, events) in self.events.iter_mut() {
            let entity_ids = events
                .iter_mut()
                .map(|x| x.execute(world))
                .collect::<Vec<_>>();
            self.observers.flush(type_id, &entity_ids, world);
        }
    }

    pub fn take(&mut self) -> EventManager {
        EventManager {
            events: self
                .events
                .drain()
                .collect::<HashMap<TypeId, Vec<Box<dyn EntityEvent>>>>(),
            observers: self.observers.take(),
        }
    }

    pub fn give(&mut self, events: EventManager) {
        self.observers = events.observers;
    }

    pub fn clear(&mut self, observers: Option<Observers>) {
        self.events.clear();
        if let Some(observers) = observers {
            self.observers = observers;
        } else {
            self.observers.clear();
        }
    }
}

impl Resource for EventManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct Observers {
    observers: HashMap<TypeId, Vec<Box<dyn Observer>>>,
}

impl Observers {
    pub fn new() -> Observers {
        Observers {
            observers: HashMap::new(),
        }
    }

    pub fn observe<T: EntityEvent>(&mut self, system: impl Observer) {
        let type_id = std::any::TypeId::of::<T>();

        self.observers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push(Box::new(system));
    }

    pub fn flush(&self, type_id: &TypeId, entity_ids: &[EntityId], world: &World) {
        if let Some(observers) = self.observers.get(type_id) {
            for observer in observers {
                observer.observe(entity_ids, world);
            }
        }
    }

    pub fn clear(&mut self) {
        self.observers.clear();
    }

    fn take(&mut self) -> Observers {
        Observers {
            observers: self
                .observers
                .drain()
                .collect::<HashMap<TypeId, Vec<Box<dyn Observer>>>>(),
        }
    }
}
