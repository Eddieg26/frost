use crate::{
    asset::{Asset, AssetImporter, ImporterRepo},
    ecs::{Component, ComponentManager, Resource, ResourceManager},
    service::{Service, ServiceRepo},
};
use std::collections::HashMap;

pub mod inner;

pub use inner::*;

pub struct AppBuilder {
    importers: ImporterRepo,
    services: ServiceRepo,
    resources: ResourceManager,
    components: ComponentManager,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            importers: ImporterRepo::new(),
            services: ServiceRepo::new(),
            resources: ResourceManager::new(),
            components: ComponentManager::new(HashMap::new()),
        }
    }

    pub fn with_importer<T: Asset, U: AssetImporter<T>>(mut self, importer: U) -> Self {
        self.importers.add_importer::<T, U>(importer);

        self
    }

    pub fn with_service<T: Service>(mut self, service: T) -> Self {
        self.services.register(service);

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
}
