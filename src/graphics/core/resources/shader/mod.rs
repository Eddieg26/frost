use super::material::{BlendMode, Material};
use crate::graphics::light::Light;

pub mod program;
pub mod resources;
pub mod templates;

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraData {
    view: [f32; 16],
    projection: [f32; 16],
}

impl CameraData {
    pub fn new(view: glam::Mat4, projection: glam::Mat4) -> Self {
        Self {
            view: view.to_cols_array(),
            projection: projection.to_cols_array(),
        }
    }

    pub fn identity() -> Self {
        Self {
            view: glam::Mat4::IDENTITY.to_cols_array(),
            projection: glam::Mat4::IDENTITY.to_cols_array(),
        }
    }
}

pub struct ObjectData {
    model: [f32; 16],
}

impl ObjectData {
    pub fn new(model: glam::Mat4) -> Self {
        Self {
            model: model.to_cols_array(),
        }
    }

    pub fn identity() -> Self {
        Self {
            model: glam::Mat4::IDENTITY.to_cols_array(),
        }
    }
}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct LitMaterialData {
    pub color: [f32; 4],
    pub specular: [f32; 4],
    pub metallic: [f32; 4],
    pub roughness: [f32; 4],
    pub emissive: [f32; 4],
    pub opacity: [f32; 4],
}

impl LitMaterialData {
    pub fn new() -> Self {
        Self {
            color: [1.0; 4],
            specular: [0.0; 4],
            metallic: [0.0; 4],
            roughness: [0.0; 4],
            emissive: [0.0; 4],
            opacity: [1.0; 4],
        }
    }

    pub fn from_material(material: &Material) -> LitMaterialData {
        LitMaterialData {
            color: Material::get_input_color(&Some(material.color()), [1.0; 4]),
            specular: Material::get_input_color(&material.specular(), [1.0; 4]),
            metallic: Material::get_input_color(&material.metallic(), [1.0; 4]),
            roughness: Material::get_input_color(&material.roughness(), [1.0; 4]),
            emissive: Material::get_input_color(&material.emissive(), [1.0; 4]),
            opacity: match material.blend_mode() {
                BlendMode::Opaque => [1.0; 4],
                BlendMode::Translucent => Material::get_input_color(&material.opacity(), [1.0; 4]),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct UnlitMaterialData {
    pub color: [f32; 4],
    pub opacity: [f32; 4],
}

impl UnlitMaterialData {
    pub fn new() -> Self {
        Self {
            color: [1.0; 4],
            opacity: [1.0; 4],
        }
    }

    pub fn from_material(material: &Material) -> UnlitMaterialData {
        UnlitMaterialData {
            color: Material::get_input_color(&Some(material.color()), [1.0; 4]),
            opacity: match material.blend_mode() {
                BlendMode::Opaque => [1.0; 4],
                BlendMode::Translucent => Material::get_input_color(&material.opacity(), [1.0; 4]),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct LightData {
    position: [f32; 3],
    _padding: u32,
    direction: [f32; 3],
    _padding2: u32,
    color: [f32; 3],
    _padding3: u32,
    intensity: f32,
    range: f32,
    spot_angle: f32,
    kind: u32,
}

impl LightData {
    pub fn new(light: &Light, transform: glam::Mat4) -> LightData {
        let (_, rotation, position) = transform.to_scale_rotation_translation();
        let direction = rotation.xyz().normalize();

        LightData {
            position: position.into(),
            direction: direction.into(),
            color: light.color.into(),
            intensity: light.intensity,
            range: light.range,
            spot_angle: light.spot_angle,
            kind: light.kind as u32,
            _padding: 0,
            _padding2: 0,
            _padding3: 0,
        }
    }
}
