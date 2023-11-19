use std::{collections::HashMap, rc::Rc};
use wgpu::util::DeviceExt;

use crate::{
    graphics::{
        BufferId, GpuDevice, MaterialId, MaterialInfo, Mesh, MeshId, Texture, TextureId, Vertex,
    },
    service::Service,
};

pub struct Graphics {
    device: Rc<GpuDevice>,
    buffers: HashMap<BufferId, wgpu::Buffer>,
    textures: HashMap<TextureId, wgpu::Texture>,
    meshes: HashMap<MeshId, Mesh>,
}

impl Graphics {
    pub fn new(device: Rc<GpuDevice>) -> Self {
        Self {
            device,
            buffers: HashMap::new(),
            textures: HashMap::new(),
            meshes: HashMap::new(),
        }
    }

    pub fn buffer(&self, id: &BufferId) -> Option<&wgpu::Buffer> {
        self.buffers.get(id)
    }

    pub fn texture(&self, id: &TextureId) -> Option<&wgpu::Texture> {
        self.textures.get(id)
    }

    pub fn mesh(&self, id: &MeshId) -> Option<&Mesh> {
        self.meshes.get(id)
    }
}

impl Graphics {
    pub fn create_vertex_buffer(&mut self, id: &BufferId, vertices: &Vec<Vertex>) {
        let vertex_buffer =
            self.device
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
            self.device
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
            self.device
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Uniform Buffer"),
                    contents: buffer,
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        self.buffers.insert(id.clone(), uniform_buffer);
    }

    pub fn create_texture(&mut self, id: &TextureId, texture: &dyn Texture) {
        let gpu_texture = self
            .device
            .device()
            .create_texture(&wgpu::TextureDescriptor {
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

        self.device.queue().write_texture(
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

    pub fn create_material(&mut self, id: &MaterialId, info: MaterialInfo) {
        println!("create_material: {:?} {:?}", id, info);
    }

    pub fn create_mesh(&mut self, id: &MeshId, vertices: &Vec<Vertex>, indices: &Vec<u32>) {
        self.meshes.insert(
            id.clone(),
            Mesh::new(self.device.device(), &vertices, &indices),
        );
    }
}

impl Service for Graphics {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
