use ecs::{world::Write, EntityId, World};

use crate::{ecs::world::events::CreateEntity, game::Game};

pub mod asset;
pub mod ecs;
pub mod game;
pub mod graphics;
pub mod scene;
pub mod schedule;
pub mod shared;
pub mod spatial;

pub struct MainScene;

impl scene::Scene for MainScene {
    fn name(&self) -> &str {
        todo!()
    }

    fn scheduler(&self) -> schedule::Scheduler {
        let schedule = schedule::Schedule::new().add_system(test_system).flush();

        schedule::builder()
            .add_schedule(schedule::ScenePhase::Update, schedule)
            .build()
    }
}

pub struct Player {
    pub name: String,
    pub health: u32,
}

impl ecs::Component for Player {}

fn test_system(world: &World) {
    let entity = CreateEntity::new().with(Player {
        name: "Player".to_string(),
        health: 100,
    });

    world.spawn(entity);
    println!("Hello, world!");
}

fn main() {
    Game::new().run::<MainScene>()
}

pub struct Goblin {
    pub health: u32,
}

impl ecs::Component for Goblin {}

pub fn player_system(world: &World) {
    let _query = ecs::world::Query::<(EntityId, Write<Player>, Option<Goblin>)>::new(world);

    // for (id, player, goblin) in query {}
}
