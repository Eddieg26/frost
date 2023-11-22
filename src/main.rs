use crate::game::Game;

pub mod asset;
pub mod ecs;
pub mod game;
pub mod graphics;
pub mod scene;
pub mod shared;

pub struct MainScene;

impl scene::Scene for MainScene {
    fn name(&self) -> &str {
        todo!()
    }

    fn start(&self, world: &ecs::World) {
        todo!()
    }

    fn end(&self, world: &ecs::World) {
        todo!()
    }

    fn update(&self, world: &ecs::World) {
        todo!()
    }
}

fn main() {
    Game::new().run::<MainScene>()
}
