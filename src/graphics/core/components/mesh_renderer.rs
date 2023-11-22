use crate::{
    ecs::Component,
    graphics::{MaterialId, MeshId},
};

pub struct MeshElement {
    pub mesh: MeshId,
    pub material: MaterialId,
}

pub struct MeshRenderer {
    pub elements: Vec<MeshElement>,
}

impl MeshRenderer {
    pub fn new(elements: Vec<MeshElement>) -> Self {
        Self { elements }
    }
}

impl Component for MeshRenderer {}
