use super::{MaterialId, MeshId};
use crate::{
    graphics::light::{Light2D, Light3D, LightRef},
    shared::{Bounds, Rect},
    spatial::{
        octtree::{Object3D, OctTree},
        quadtree::QuadTree,
    },
};

pub struct DrawMesh {
    transform: glam::Mat4,
    mesh: MeshId,
    materials: Vec<MaterialId>,
    bounds: Bounds,
}

impl DrawMesh {
    pub fn new(
        transform: glam::Mat4,
        mesh: MeshId,
        materials: Vec<MaterialId>,
        bounds: Bounds,
    ) -> Self {
        Self {
            transform,
            mesh,
            materials,
            bounds,
        }
    }

    pub fn transform(&self) -> &glam::Mat4 {
        &self.transform
    }

    pub fn mesh(&self) -> MeshId {
        self.mesh
    }

    pub fn materials(&self) -> &[MaterialId] {
        &self.materials
    }

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

impl Object3D for DrawMesh {
    fn bounds(&self) -> &Bounds {
        &self.bounds
    }
}

pub struct RenderScene {
    meshes: OctTree<DrawMesh>,
    lights3d: OctTree<Light3D>,
    lights2d: QuadTree<Light2D>,
}

impl RenderScene {
    pub fn new() -> RenderScene {
        RenderScene {
            meshes: OctTree::new(Bounds::MAX, 8, 8), 
            lights3d: OctTree::new(Bounds::MAX, 8, 8),
            lights2d: QuadTree::new(Rect::MAX, 8, 8),
        }
    }

    pub fn add_mesh(&mut self, mesh: DrawMesh) {
        self.meshes.insert(mesh);
    }

    pub fn add_light(&mut self, light: LightRef) {
        match light {
            LightRef::D2(light) => self.lights2d.insert(light),
            LightRef::D3(light) => self.lights3d.insert(light),
        }
    }

    pub fn clear(&mut self) {
        self.meshes.clear();
        self.lights2d.clear();
        self.lights3d.clear();
    }
}
