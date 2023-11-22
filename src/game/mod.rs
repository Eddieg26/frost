use crate::{
    asset::{Asset, AssetImporter, ImporterRepo},
    ecs::{Component, ComponentManager, Resource, ResourceManager},
    scene::{Scene, ScenesBuilder},
};
use std::collections::HashMap;

pub mod inner;
pub mod time;

pub use inner::*;
pub use time::*;

pub struct GameBuilder {
    importers: ImporterRepo,
    resources: ResourceManager,
    components: ComponentManager,
    scenes: ScenesBuilder,
}

impl GameBuilder {
    pub fn new() -> Self {
        Self {
            importers: ImporterRepo::new(),
            resources: ResourceManager::new(),
            components: ComponentManager::new(HashMap::new()),
            scenes: ScenesBuilder::new(),
        }
    }

    pub fn with_importer<T: Asset, U: AssetImporter<T>>(mut self, importer: U) -> Self {
        self.importers.add_importer::<T, U>(importer);

        self
    }

    pub fn with_resource<T: Resource>(mut self, resource: T) -> Self {
        self.resources.register(resource);

        self
    }

    pub fn with_component<T: Component>(mut self) -> Self {
        self.components.register::<T>();

        self
    }

    pub fn with_scene<T: Scene>(mut self, scene: T) -> Self {
        self.scenes.add_scene(scene);

        self
    }

    pub fn run<T: Scene>(self) {
        GameRunner::run::<T>(self)
    }
}
