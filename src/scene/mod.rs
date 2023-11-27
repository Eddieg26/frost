use crate::{
    ecs::{observer::Observers, Resource},
    schedule::Scheduler,
    shared::ResourceType,
};
use std::{any::TypeId, collections::HashMap};
pub type SceneId = ResourceType;

pub trait Scene: 'static {
    fn name(&self) -> &str;
    fn scheduler(&self) -> Scheduler;
    fn observers(&self) -> Option<Observers> {
        None
    }
}

pub struct SceneManager {
    scenes: HashMap<SceneId, Box<dyn Scene>>,
    current: SceneId,
    next: Option<SceneId>,
    quitting: bool,
}

impl SceneManager {
    pub fn new(scenes: HashMap<SceneId, Box<dyn Scene>>, current: SceneId) -> Self {
        Self {
            scenes,
            current,
            next: None,
            quitting: false,
        }
    }

    pub fn current(&self) -> SceneId {
        self.current
    }

    pub fn current_scene(&self) -> &Box<dyn Scene> {
        self.scenes.get(&self.current).expect("Scene doesn't exist")
    }

    pub fn next(&self) -> Option<SceneId> {
        self.next
    }

    pub fn next_scene(&self) -> Option<&Box<dyn Scene>> {
        self.next
            .map(|id| self.scenes.get(&id).expect("Scene doesn't exist"))
    }

    pub fn set_next(&mut self, id: SceneId) {
        self.next = Some(id);
    }

    pub fn quit(&mut self) {
        self.quitting = true;
    }

    pub fn quitting(&self) -> bool {
        self.quitting
    }

    pub fn update(&mut self) -> Option<Scheduler> {
        if self.quitting {
            return None;
        }

        if let Some(next) = self.next() {
            self.current = next;
            Some(self.current_scene().scheduler())
        } else {
            None
        }
    }
}

impl Resource for SceneManager {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

pub struct ScenesBuilder {
    scenes: HashMap<SceneId, Box<dyn Scene>>,
}

impl ScenesBuilder {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
        }
    }

    pub fn add_scene<T: Scene>(&mut self, scene: T) {
        let id = TypeId::of::<T>().into();
        self.scenes.insert(id, Box::new(scene));
    }

    pub fn build<T: Scene>(self) -> SceneManager {
        let id = TypeId::of::<T>().into();
        SceneManager::new(self.scenes, id)
    }

    pub fn all(&self) -> &HashMap<SceneId, Box<dyn Scene>> {
        &self.scenes
    }
}
