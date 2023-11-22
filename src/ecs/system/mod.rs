use super::{
    component::Component,
    world::{
        query::{Query, Write},
        World,
    },
    EntityId,
};

pub trait System {
    fn run(world: &World);
}

pub struct Player {
    pub health: u32,
}

impl Component for Player {}

pub struct Goblin {
    pub health: u32,
}

impl Component for Goblin {}

pub struct PlayerSystem {}

impl System for PlayerSystem {
    fn run(world: &World) {
        let _query = Query::<(EntityId, Write<Player>, Option<Goblin>)>::new(world);
        // events.add(CreateEntity::new().with(world, Player { health: 5 }));
        // events.add(DestroyEntity::new(EntityId::new(None)));
        // events.add(DestroyComponent::<Player>::new(EntityId(0)));

        // for (player, goblin) in query {}
    }
}
