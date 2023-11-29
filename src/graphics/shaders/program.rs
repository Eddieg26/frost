use crate::graphics::{
    self,
    material::{
        BlendMode, LitMaterialUniform, Material, MaterialUniform, ShaderInput, ShaderModel,
        UnLitMaterialUniform,
    },
    texture::Texture,
    vertex::Vertex,
    Graphics,
};

use super::{layout::ShaderLayout, templates};

pub struct MaterialBindGroup {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl MaterialBindGroup {
    pub fn new(buffer: wgpu::Buffer, bind_group: wgpu::BindGroup) -> MaterialBindGroup {
        Self { buffer, bind_group }
    }

    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

pub struct MaterialLayout {
    buffer: wgpu::Buffer,
    layout: wgpu::BindGroupLayout,
}

impl MaterialLayout {
    pub fn new(material: &Material, graphics: &Graphics) -> MaterialLayout {
        let mut entries = vec![];
        entries.push(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        for idx in 1..Self::get_texture_count(material) {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: idx * 2 + 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            });
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: (idx * 2) + 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            });
        }

        let layout =
            graphics
                .gpu()
                .device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &entries,
                });

        let material_uniform = Self::get_material_uniform(material);
        let buffer = graphics.create_uniform_buffer(material_uniform.to_bytes());

        Self { buffer, layout }
    }

    pub fn get_material_uniform(material: &Material) -> Box<dyn MaterialUniform> {
        match material.shader_model() {
            ShaderModel::Lit => Box::new(LitMaterialUniform::from_material(material)),
            ShaderModel::Unlit => Box::new(UnLitMaterialUniform::from_material(material)),
        }
    }

    pub fn get_input_textures<'a>(
        material: &'a Material,
        graphics: &'a Graphics,
    ) -> Vec<&'a dyn Texture> {
        let mut textures = Vec::new();
        if let Some(color) = Self::get_input_texture(material.color(), graphics) {
            textures.push(color);
        }
        if let Some(opacity) = Self::get_input_texture(material.opacity(), graphics) {
            textures.push(opacity);
        }
        if material.shader_model() == ShaderModel::Lit {
            if let Some(specular) = Self::get_input_texture(material.specular(), graphics) {
                textures.push(specular);
            }
            if let Some(normal) = Self::get_input_texture(material.normal(), graphics) {
                textures.push(normal);
            }
            if let Some(metallic) = Self::get_input_texture(material.metallic(), graphics) {
                textures.push(metallic);
            }
            if let Some(roughness) = Self::get_input_texture(material.roughness(), graphics) {
                textures.push(roughness);
            }
            if let Some(emissive) = Self::get_input_texture(material.emissive(), graphics) {
                textures.push(emissive);
            }
        }

        textures
    }

    fn create_bind_group(&self, material: &Material, graphics: &Graphics) -> MaterialBindGroup {
        let textures = Self::get_input_textures(material, graphics);
        let mut entries = vec![];

        entries.push(wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer: &self.buffer,
                offset: 0,
                size: None,
            }),
        });

        for (i, texture) in textures.iter().enumerate() {
            entries.push(wgpu::BindGroupEntry {
                binding: (i * 2) as u32 + 1,
                resource: wgpu::BindingResource::TextureView(texture.view()),
            });
            entries.push(wgpu::BindGroupEntry {
                binding: (i * 2) as u32 + 2,
                resource: wgpu::BindingResource::Sampler(texture.sampler()),
            });
        }

        let material_uniform = Self::get_material_uniform(material);
        let buffer = graphics.create_uniform_buffer(material_uniform.to_bytes());

        let bind_group = graphics
            .gpu()
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.layout,
                entries: &entries,
                label: None,
            });

        MaterialBindGroup::new(buffer, bind_group)
    }

    fn get_input_texture<'a>(
        input: Option<&'a ShaderInput>,
        graphics: &'a Graphics,
    ) -> Option<&'a dyn Texture> {
        match input {
            Some(ShaderInput::Texture(texture_id)) => graphics.dyn_texture(texture_id),
            _ => None,
        }
    }

    fn get_texture_count(material: &Material) -> u32 {
        let mut count = 0;

        material.color().and_then(|_| Some(count += 1));
        material.opacity().and_then(|_| Some(count += 1));
        if material.shader_model() == ShaderModel::Lit {
            material.specular().and_then(|_| Some(count += 1));
            material.normal().and_then(|_| Some(count += 1));
            material.metallic().and_then(|_| Some(count += 1));
            material.roughness().and_then(|_| Some(count += 1));
            material.emissive().and_then(|_| Some(count += 1));
        }
        count
    }
}

pub struct ObjectData {
    pub model: [f32; 16],
}

pub struct ShaderProgram {
    layout: MaterialLayout,
}

impl ShaderProgram {
    pub fn new(material: &Material, graphics: &Graphics) -> Self {
        let shader_layout = ShaderLayout::from_material(material);
        let material_layout = MaterialLayout::new(material, graphics);

        let global_group_layout =
            graphics
                .gpu()
                .device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::VERTEX,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                });

        let push_constant_range: wgpu::PushConstantRange = wgpu::PushConstantRange {
            range: 0..64,
            stages: wgpu::ShaderStages::VERTEX,
        };

        let pipeline_layout =
            graphics
                .gpu()
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&global_group_layout, &material_layout.layout],
                    push_constant_ranges: &[push_constant_range],
                });

        let fragment_shader_str = match material.shader_model() {
            ShaderModel::Lit => templates::forward::forward_shader_template(&shader_layout, 100),
            ShaderModel::Unlit => templates::unlit::unlit_shader_template(&shader_layout),
        };

        let fragment_shader =
            graphics
                .gpu()
                .device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(fragment_shader_str.into()),
                });

        let render_pipeline =
            graphics
                .gpu()
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_layout.vertex_shader,
                        entry_point: "main",
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    offset: 0,
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float32x3,
                                },
                                wgpu::VertexAttribute {
                                    offset: 16,
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32x3,
                                },
                                wgpu::VertexAttribute {
                                    offset: 32,
                                    shader_location: 2,
                                    format: wgpu::VertexFormat::Float32x2,
                                },
                            ],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &fragment_shader,
                        entry_point: "main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Depth24Plus,
                            blend: Some(Self::get_blend_state(material.blend_mode())),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                });

        Self {
            layout: material_layout,
        }
    }

    pub fn create_bind_group(&self, material: &Material, graphics: &Graphics) -> MaterialBindGroup {
        self.layout.create_bind_group(material, graphics)
    }

    fn get_blend_state(blend_mode: BlendMode) -> wgpu::BlendState {
        match blend_mode {
            BlendMode::Opaque => wgpu::BlendState::REPLACE,
            BlendMode::Translucent => wgpu::BlendState::ALPHA_BLENDING,
        }
    }
}

// Object that extracts textures and material attributes from a material
// Can Also be used to create a material bind group
