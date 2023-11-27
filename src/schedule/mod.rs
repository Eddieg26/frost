use crate::ecs::{observer::EventManager, System, World};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScenePhase {
    Start,
    Update,
    PostUpdate,
    PreRender,
    PostRender,
    End,
}

fn flush(world: &World) {
    let mut events = world.resource_mut::<EventManager>().take();
    events.flush(world);
    world.resource_mut::<EventManager>().give(events);
}

pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system<T: System + 'static>(mut self, system: T) -> Self {
        self.systems.push(Box::new(system));

        self
    }

    pub fn flush(self) -> Self {
        self.add_system(flush)
    }

    pub fn run(&self, world: &World) {
        for system in &self.systems {
            system.run(&world);
        }
    }
}

pub struct SchedulerBuilder {
    schedules: HashMap<ScenePhase, Vec<Schedule>>,
}

impl SchedulerBuilder {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
        }
    }

    pub fn add_schedule(mut self, phase: ScenePhase, schedule: Schedule) -> Self {
        self.schedules
            .entry(phase)
            .or_insert_with(Vec::new)
            .push(schedule);

        self
    }

    pub fn build(self) -> Scheduler {
        Scheduler {
            schedules: self.schedules,
        }
    }
}
pub struct Scheduler {
    schedules: HashMap<ScenePhase, Vec<Schedule>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
        }
    }

    pub fn run(&mut self, phase: ScenePhase, world: &World) {
        if let Some(schedules) = self.schedules.get_mut(&phase) {
            for schedule in schedules {
                schedule.run(world);
            }
        }
    }
}

pub fn builder() -> SchedulerBuilder {
    SchedulerBuilder::new()
}
