pub mod entity;

use super::World;

pub struct WorldEvents {
    events: Vec<Box<dyn Event>>,
}

impl WorldEvents {
    pub fn new() -> WorldEvents {
        WorldEvents { events: vec![] }
    }

    pub fn add<T: Event>(&mut self, event: T) {
        let event = Box::new(event);
        self.events.push(event);
    }

    pub fn execute(&mut self, world: &World) {
        while let Some(mut event) = self.events.pop() {
            event.execute(world);
        }
    }
}

pub trait Event: 'static {
    fn execute(&mut self, world: &World);
}
