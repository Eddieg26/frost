use super::{
    gpu::Gpu,
    material::Material,
    mesh::SubMesh,
    shader::{program::ShaderProgram, resources::ShaderResources},
    vertex::Vertex,
    BufferId, DrawMesh, MaterialId, MeshId, RenderScene, TextureId,
};
use crate::{
    ecs::Resource,
    graphics::{
        light::LightRef,
        material::MaterialInfo,
        mesh::Mesh,
        texture::{Texture, TextureInfo},
    },
};
use std::{collections::HashMap, rc::Rc};
use wgpu::util::DeviceExt;

pub struct Graphics {
    gpu: Rc<Gpu>,
    scene: RenderScene,
    buffers: HashMap<BufferId, wgpu::Buffer>,
    textures: HashMap<TextureId, Box<dyn Texture>>,
    meshes: HashMap<MeshId, Mesh>,
    materials: HashMap<MaterialId, Material>,
    shader_programs: HashMap<Material, ShaderProgram>,
    shader_resources: ShaderResources,
    config: Config,
}

impl Graphics {
    pub fn new(gpu: Rc<Gpu>, config: Config) -> Self {
        let shader_resources = ShaderResources::new(gpu.device(), 100);

        Self {
            gpu,
            scene: RenderScene::new(),
            buffers: HashMap::new(),
            textures: HashMap::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
            shader_programs: HashMap::new(),
            shader_resources,
            config,
        }
    }

    pub fn gpu(&self) -> &Gpu {
        &self.gpu
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn buffer(&self, id: &BufferId) -> Option<&wgpu::Buffer> {
        self.buffers.get(id)
    }

    pub fn texture<T: Texture>(&self, id: &TextureId) -> Option<&T> {
        self.textures
            .get(id)
            .and_then(|t| t.as_any().downcast_ref::<T>())
    }

    pub fn dyn_texture(&self, id: &TextureId) -> Option<&dyn Texture> {
        self.textures.get(id).and_then(|t| Some(t.as_ref()))
    }

    pub fn mesh(&self, id: &MeshId) -> Option<&Mesh> {
        self.meshes.get(id)
    }

    pub fn material(&self, id: &MaterialId) -> Option<&Material> {
        self.materials.get(id)
    }

    pub fn shader_program(&self, material: &Material) -> Option<&ShaderProgram> {
        self.shader_programs.get(material)
    }

    pub fn shader_resources(&self) -> &ShaderResources {
        &self.shader_resources
    }

    pub(super) fn scene(&self) -> &RenderScene {
        &self.scene
    }

    pub(super) fn scene_mut(&mut self) -> &mut RenderScene {
        &mut self.scene
    }

    pub fn add_buffer(&mut self, id: &BufferId, buffer: wgpu::Buffer) {
        self.buffers.insert(id.clone(), buffer);
    }

    pub fn add_texture<T: Texture>(&mut self, id: &TextureId, texture: T) {
        self.textures.insert(id.clone(), Box::new(texture));
    }

    pub fn add_mesh(&mut self, id: &MeshId, mesh: Mesh) {
        self.meshes.insert(id.clone(), mesh);
    }

    pub fn add_material(&mut self, id: &MaterialId, material: Material) {
        self.materials.insert(id.clone(), material);
    }

    pub fn create_vertex_buffer(&self, vertices: &Vec<Vertex>) -> wgpu::Buffer {
        self.gpu
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
    }

    pub fn create_index_buffer(&self, indices: &Vec<u32>) -> wgpu::Buffer {
        self.gpu
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            })
    }

    pub fn create_uniform_buffer(&self, buffer: &[u8]) -> wgpu::Buffer {
        self.gpu
            .device()
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: buffer,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
    }

    pub fn create_texture(&self, info: &TextureInfo) -> wgpu::TextureView {
        let gpu_texture = self.gpu.device().create_texture(&wgpu::TextureDescriptor {
            dimension: info.dimension,
            format: info.format,
            label: None,
            mip_level_count: if info.mipmaps { 1 } else { 0 },
            sample_count: 1,
            size: wgpu::Extent3d {
                depth_or_array_layers: info.depth,
                height: info.height,
                width: info.width,
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
            &info.pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(info.width * 4),
                rows_per_image: Some(info.height),
            },
            wgpu::Extent3d {
                depth_or_array_layers: info.depth,
                height: info.height,
                width: info.width,
            },
        );

        gpu_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn create_sampler(&self, info: &TextureInfo) -> wgpu::Sampler {
        self.gpu.device().create_sampler(&wgpu::SamplerDescriptor {
            label: None,
            address_mode_u: info.wrap_mode.into(),
            address_mode_v: info.wrap_mode.into(),
            address_mode_w: info.wrap_mode.into(),
            mag_filter: info.filter_mode.into(),
            min_filter: info.filter_mode.into(),
            mipmap_filter: info.filter_mode.into(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        })
    }

    pub fn create_material(&mut self, info: &MaterialInfo) -> Material {
        let material = info.build();
        if !self.shader_programs.contains_key(&material) {
            let shader_program = ShaderProgram::new(self, &material);
            self.shader_programs
                .insert(material.clone(), shader_program);
        }

        material
    }

    pub fn create_mesh(
        &self,
        vertices: &Vec<Vertex>,
        indices: &Vec<u32>,
        submeshes: &[SubMesh],
    ) -> Mesh {
        Mesh::new(self.gpu.device(), &vertices, &indices, &submeshes)
    }

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

pub struct Config {
    color_format: wgpu::TextureFormat,
    depth_format: wgpu::TextureFormat,
}

impl Config {
    pub fn new(color_format: wgpu::TextureFormat, depth_format: wgpu::TextureFormat) -> Self {
        Self {
            color_format,
            depth_format,
        }
    }

    pub fn color_format(&self) -> wgpu::TextureFormat {
        self.color_format
    }

    pub fn depth_format(&self) -> wgpu::TextureFormat {
        self.depth_format
    }
}
