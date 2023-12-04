use super::templates::{
    forward::ForwardShaderTemplate,
    layout::{MaterialBindGroupLayout, ShaderLayout},
    unlit::UnlitShaderTemplate,
};
use crate::graphics::{
    material::{Material, ShaderModel},
    Graphics, MaterialId,
};
use std::collections::HashMap;

pub struct ShaderProgram {
    model: ShaderModel,
    pipeline: wgpu::RenderPipeline,
    layout: MaterialBindGroupLayout,
    material_bind_groups: HashMap<MaterialId, wgpu::BindGroup>,
}

impl ShaderProgram {
    pub fn new(graphics: &Graphics, material: &Material) -> ShaderProgram {
        let shader_layout = ShaderLayout::from_material(material);
        let material_layout =
            graphics
                .gpu()
                .device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("material_layout"),
                    entries: &shader_layout.bind_group_layout_entries(),
                });

        let shader_module = match material.shader_model() {
            ShaderModel::Lit => {
                ForwardShaderTemplate::create_shader(graphics.gpu().device(), &shader_layout, 100)
            }
            ShaderModel::Unlit => {
                UnlitShaderTemplate::create_shader(graphics.gpu().device(), &shader_layout)
            }
        };

        let pipeline = Self::create_pipeline(graphics, material, &material_layout, &shader_module);
        let layout = MaterialBindGroupLayout::from_material(material_layout, material);

        ShaderProgram {
            pipeline,
            layout,
            model: material.shader_model(),
            material_bind_groups: HashMap::new(),
        }
    }

    pub fn add_material_bind_group(
        &mut self,
        graphics: &Graphics,
        id: MaterialId,
        material: &Material,
    ) {
        if !self.material_bind_groups.contains_key(&id) {
            let bind_group = self.layout.create_bind_group(graphics, material);
            self.material_bind_groups.insert(id, bind_group);
        }
    }

    pub fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    pub fn model(&self) -> ShaderModel {
        self.model
    }

    pub fn bind_group(&self, id: &MaterialId) -> Option<&wgpu::BindGroup> {
        self.material_bind_groups.get(id)
    }

    fn create_pipeline(
        graphics: &Graphics,
        material: &Material,
        material_layout: &wgpu::BindGroupLayout,
        shader_module: &wgpu::ShaderModule,
    ) -> wgpu::RenderPipeline {
        let global_layout = match material.shader_model() {
            ShaderModel::Lit => graphics.shader_resources().lit_global_layout(),
            ShaderModel::Unlit => graphics.shader_resources().unlit_global_layout(),
        };
        let object_layout = graphics.shader_resources().object_layout();

        let pipeline_layout =
            graphics
                .gpu()
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("shader_pipeline_layout"),
                    bind_group_layouts: &[global_layout, object_layout, &material_layout],
                    push_constant_ranges: &[],
                });

        let pipeline =
            graphics
                .gpu()
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader_module,
                        entry_point: "vs_main",
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<f32>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttribute {
                                    offset: 0,
                                    shader_location: 0,
                                    format: wgpu::VertexFormat::Float32x3,
                                },
                                wgpu::VertexAttribute {
                                    offset: std::mem::size_of::<[f32; 3]>() as u64,
                                    shader_location: 1,
                                    format: wgpu::VertexFormat::Float32x3,
                                },
                                wgpu::VertexAttribute {
                                    offset: std::mem::size_of::<[f32; 6]>() as u64,
                                    shader_location: 2,
                                    format: wgpu::VertexFormat::Float32x2,
                                },
                            ],
                        }],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader_module,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: graphics.config().color_format(),
                            blend: Some(material.blend_mode().into()),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: graphics.config().depth_format(),
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    primitive: wgpu::PrimitiveState::default(),
                    multisample: wgpu::MultisampleState::default(),
                    multiview: None,
                });

        pipeline
    }
}
