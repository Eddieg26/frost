use crate::{
    ecs::{Resource, World},
    shared::ResourceType,
};
use std::{any::TypeId, cell::RefCell, collections::HashMap, rc::Rc};
pub type SceneId = ResourceType;

pub trait Scene: 'static {
    fn name(&self) -> &str;
    fn start(&self, world: &World);
    fn update(&self, world: &World);
    fn end(&self, world: &World);
}

pub struct SceneManager {
    scenes: HashMap<SceneId, Rc<RefCell<Box<dyn Scene>>>>,
    current: SceneId,
    next: Option<SceneId>,
    quitting: bool,
}

impl SceneManager {
    pub fn new(scenes: HashMap<SceneId, Rc<RefCell<Box<dyn Scene>>>>, current: SceneId) -> Self {
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

    pub fn current_scene(&self) -> Rc<RefCell<Box<dyn Scene>>> {
        self.scenes.get(&self.current).unwrap().clone()
    }

    pub fn next(&self) -> Option<SceneId> {
        self.next
    }

    pub fn next_scene(&self) -> Option<Rc<RefCell<Box<dyn Scene>>>> {
        self.next.map(|id| self.scenes.get(&id).unwrap().clone())
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

    pub fn update(&mut self, world: &World) {
        if self.quitting {
            return;
        }

        if let Some(next) = self.next() {
            self.current_scene().borrow_mut().end(world);
            self.current = next;
            self.current_scene().borrow_mut().start(world);
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
    scenes: HashMap<SceneId, Rc<RefCell<Box<dyn Scene>>>>,
}

impl ScenesBuilder {
    pub fn new() -> Self {
        Self {
            scenes: HashMap::new(),
        }
    }

    pub fn add_scene<T: Scene>(&mut self, scene: T) {
        let id = TypeId::of::<T>().into();
        self.scenes
            .insert(id, Rc::new(RefCell::new(Box::new(scene))));
    }

    pub fn build<T: Scene>(self) -> SceneManager {
        let id = TypeId::of::<T>().into();
        SceneManager::new(self.scenes, id)
    }

    pub fn all(&self) -> &HashMap<SceneId, Rc<RefCell<Box<dyn Scene>>>> {
        &self.scenes
    }
}
