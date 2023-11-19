use super::AppBuilder;
use crate::{
    asset::AssetDatabase, builtin::services::graphics::Graphics, ecs::World,
    graphics::engine::GraphicsEngine,
};
use std::path::Path;
use winit::window::Window;

pub struct App {
    graphics: GraphicsEngine,
    world: World,
}

impl App {
    pub fn new() -> AppBuilder {
        AppBuilder::new()
    }

    async fn build(window: Window, builder: AppBuilder) -> App {
        let graphics = GraphicsEngine::new(window).await;

        let importers = builder.importers;
        let components = builder.components;
        let mut services = builder.services;
        let mut resources = builder.resources;

        services.register(Graphics::new(graphics.device().clone()));
        resources.register(AssetDatabase::new());
        resources.register(services.clone());

        AssetDatabase::load(&Path::new("./assets"), &resources, &importers);

        let world = World::new(components, resources);

        App { graphics, world }
    }
}

impl App {}
