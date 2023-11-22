use crate::{
    ecs::Component,
    graphics::{MaterialId, SpriteId},
};

pub struct SpriteRenderer {
    pub sprite: SpriteId,
    pub material: MaterialId,
}

impl SpriteRenderer {
    pub fn new(sprite: SpriteId, material: MaterialId) -> Self {
        Self { sprite, material }
    }
}

impl Component for SpriteRenderer {}
