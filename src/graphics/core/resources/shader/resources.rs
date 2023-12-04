use super::{CameraData, LightData, LitMaterialData, ObjectData, UnlitMaterialData};

pub struct ShaderResources {
    unlit_global_layout: wgpu::BindGroupLayout,
    lit_global_layout: wgpu::BindGroupLayout,
    object_layout: wgpu::BindGroupLayout,
    lit_material: wgpu::Buffer,
    unlit_material: wgpu::Buffer,
    object: wgpu::Buffer,
    camera: wgpu::Buffer,
    lights: wgpu::Buffer,
}

impl ShaderResources {
    pub fn new(device: &wgpu::Device, max_light_count: usize) -> ShaderResources {
        let lit_material = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lit_material"),
            size: std::mem::size_of::<LitMaterialData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let unlit_material = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("unlit_material"),
            size: std::mem::size_of::<UnlitMaterialData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let object = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("object"),
            size: std::mem::size_of::<ObjectData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera"),
            size: std::mem::size_of::<CameraData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let lights = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("lights"),
            size: std::mem::size_of::<LightData>() as u64 * max_light_count as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let lit_global_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("lit_global_layout"),
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
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let unlit_global_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("unlit_global_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let object_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("object_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        ShaderResources {
            lit_global_layout,
            unlit_global_layout,
            object_layout,
            lit_material,
            unlit_material,
            object,
            camera,
            lights,
        }
    }

    pub fn lit_global_layout(&self) -> &wgpu::BindGroupLayout {
        &self.lit_global_layout
    }

    pub fn unlit_global_layout(&self) -> &wgpu::BindGroupLayout {
        &self.unlit_global_layout
    }

    pub fn object_layout(&self) -> &wgpu::BindGroupLayout {
        &self.object_layout
    }

    pub fn lit_material(&self) -> &wgpu::Buffer {
        &self.lit_material
    }

    pub fn unlit_material(&self) -> &wgpu::Buffer {
        &self.unlit_material
    }

    pub fn object(&self) -> &wgpu::Buffer {
        &self.object
    }

    pub fn camera(&self) -> &wgpu::Buffer {
        &self.camera
    }

    pub fn lights(&self) -> &wgpu::Buffer {
        &self.lights
    }
}
