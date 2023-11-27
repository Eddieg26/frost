use super::{
    gpu::Gpu, mesh::SubMesh, vertex::Vertex, BufferId, DrawMesh, MaterialId, MeshId, RenderScene,
    TextureId,
};
use crate::{
    ecs::Resource,
    graphics::{light::LightRef, material::MaterialInfo, mesh::Mesh, texture::Texture},
};
use std::{collections::HashMap, rc::Rc};
use wgpu::util::DeviceExt;

pub struct Graphics {
    gpu: Rc<Gpu>,
    buffers: HashMap<BufferId, wgpu::Buffer>,
    textures: HashMap<TextureId, wgpu::Texture>,
    meshes: HashMap<MeshId, Mesh>,
    scene: RenderScene,
}

impl Graphics {
    pub fn new(gpu: Rc<Gpu>) -> Self {
        Self {
            gpu,
            buffers: HashMap::new(),
            textures: HashMap::new(),
            meshes: HashMap::new(),
            scene: RenderScene::new(),
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }
}

impl Graphics {
    pub fn buffer(&self, id: &BufferId) -> Option<&wgpu::Buffer> {
        self.buffers.get(id)
    }

    pub fn texture(&self, id: &TextureId) -> Option<&wgpu::Texture> {
        self.textures.get(id)
    }

    pub fn mesh(&self, id: &MeshId) -> Option<&Mesh> {
        self.meshes.get(id)
    }

    pub(super) fn scene(&self) -> &RenderScene {
        &self.scene
    }

    pub(super) fn scene_mut(&mut self) -> &mut RenderScene {
        &mut self.scene
    }

    pub fn create_vertex_buffer(&mut self, id: &BufferId, vertices: &Vec<Vertex>) {
        let vertex_buffer =
            self.gpu
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        self.buffers.insert(id.clone(), vertex_buffer);
    }

    pub fn create_index_buffer(&mut self, id: &BufferId, indices: &Vec<u32>) {
        let index_buffer =
            self.gpu
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

        self.buffers.insert(id.clone(), index_buffer);
    }

    pub fn create_uniform_buffer(&mut self, id: &BufferId, buffer: &[u8]) {
        let uniform_buffer =
            self.gpu
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Uniform Buffer"),
                    contents: buffer,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        self.buffers.insert(id.clone(), uniform_buffer);
    }

    pub fn create_texture(&mut self, id: &TextureId, texture: &dyn Texture) {
        let gpu_texture = self.gpu.device().create_texture(&wgpu::TextureDescriptor {
            dimension: texture.dimension(),
            format: texture.format(),
            label: Some(&id.to_string()),
            mip_level_count: if texture.mipmaps() { 1 } else { 0 },
            sample_count: 1,
            size: wgpu::Extent3d {
                depth_or_array_layers: texture.depth(),
                height: texture.height(),
                width: texture.width(),
            },
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        self.gpu.queue().write_texture(
            wgpu::ImageCopyTexture {
                texture: &gpu_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            texture.pixels(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(texture.width() * 4),
                rows_per_image: Some(texture.height()),
            },
            wgpu::Extent3d {
                depth_or_array_layers: texture.depth(),
                height: texture.height(),
                width: texture.width(),
            },
        );

        self.textures.insert(id.clone(), gpu_texture);
    }

    pub fn create_material(&mut self, id: &MaterialId, info: &MaterialInfo) {
        println!("create_material: {:?} {:?}", id, info);
    }

    pub fn create_mesh(
        &mut self,
        id: &MeshId,
        vertices: &Vec<Vertex>,
        indices: &Vec<u32>,
        submeshes: &[SubMesh],
    ) {
        self.meshes.insert(
            id.clone(),
            Mesh::new(self.gpu.device(), &vertices, &indices, &submeshes),
        );
    }
}

impl Graphics {
    pub fn draw_mesh(&mut self, id: &MeshId, materials: Vec<MaterialId>, transform: glam::Mat4) {
        if let Some(mesh) = self.mesh(id) {
            let bounds = mesh.bounds().transform(&transform);

            self.scene
                .add_mesh(DrawMesh::new(transform, id.clone(), materials, bounds));
        }
    }

    pub fn render_light(&mut self, light: LightRef) {
        self.scene.add_light(light);
    }
}

impl Resource for Graphics {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
