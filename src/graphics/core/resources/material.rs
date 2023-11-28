use crate::graphics::{color::Color, TextureId};
use std::hash::Hash;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaderModel {
    Lit = 0,
    Unlit = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlendMode {
    Opaque = 0,
    Translucent = 1,
}

#[derive(Copy, Clone, Debug)]
pub enum ShaderInput {
    Texture(TextureId),
    Color(Color),
    Scalar(f32),
}

impl Hash for ShaderInput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Texture(_) => 0.hash(state),
            Self::Color(_) => 1.hash(state),
            Self::Scalar(_) => 2.hash(state),
        }
    }
}

impl PartialEq for ShaderInput {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Texture(_), Self::Texture(_)) => true,
            (Self::Color(_), Self::Color(_)) => true,
            (Self::Scalar(_), Self::Scalar(_)) => true,
            _ => false,
        }
    }
}

impl Eq for ShaderInput {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Material {
    shader_model: ShaderModel,
    blend_mode: BlendMode,
    color: Option<ShaderInput>,
    normal: Option<ShaderInput>,
    specular: Option<ShaderInput>,
    metallic: Option<ShaderInput>,
    roughness: Option<ShaderInput>,
    emissive: Option<ShaderInput>,
    opacity: Option<ShaderInput>,
}

impl Material {
    pub fn builder() -> MaterialInfo {
        MaterialInfo::new()
    }

    pub fn shader_model(&self) -> ShaderModel {
        self.shader_model
    }

    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    pub fn color(&self) -> Option<&ShaderInput> {
        self.color.as_ref()
    }

    pub fn specular(&self) -> Option<&ShaderInput> {
        self.specular.as_ref()
    }

    pub fn normal(&self) -> Option<&ShaderInput> {
        self.normal.as_ref()
    }

    pub fn metallic(&self) -> Option<&ShaderInput> {
        self.metallic.as_ref()
    }

    pub fn roughness(&self) -> Option<&ShaderInput> {
        self.roughness.as_ref()
    }

    pub fn emissive(&self) -> Option<&ShaderInput> {
        self.emissive.as_ref()
    }

    pub fn opacity(&self) -> Option<&ShaderInput> {
        self.opacity.as_ref()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MaterialInfo {
    shader_model: ShaderModel,
    blend_mode: BlendMode,
    color: Option<ShaderInput>,
    specular: Option<ShaderInput>,
    normal: Option<ShaderInput>,
    metallic: Option<ShaderInput>,
    roughness: Option<ShaderInput>,
    emissive: Option<ShaderInput>,
    opacity: Option<ShaderInput>,
}

impl MaterialInfo {
    pub fn new() -> Self {
        Self {
            shader_model: ShaderModel::Lit,
            blend_mode: BlendMode::Opaque,
            color: None,
            specular: None,
            normal: None,
            metallic: None,
            roughness: None,
            emissive: None,
            opacity: None,
        }
    }

    pub fn shader_model(mut self, shader_model: ShaderModel) -> Self {
        self.shader_model = shader_model;
        self
    }

    pub fn blend_mode(mut self, blend_mode: BlendMode) -> Self {
        self.blend_mode = blend_mode;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(ShaderInput::Color(color));
        self
    }

    pub fn specular(mut self, specular: TextureId) -> Self {
        self.specular = Some(ShaderInput::Texture(specular));
        self
    }

    pub fn normal(mut self, normal: TextureId) -> Self {
        self.normal = Some(ShaderInput::Texture(normal));
        self
    }

    pub fn metallic(mut self, metallic: TextureId) -> Self {
        self.metallic = Some(ShaderInput::Texture(metallic));
        self
    }

    pub fn roughness(mut self, roughness: TextureId) -> Self {
        self.roughness = Some(ShaderInput::Texture(roughness));
        self
    }

    pub fn emissive(mut self, emissive: TextureId) -> Self {
        self.emissive = Some(ShaderInput::Texture(emissive));
        self
    }

    pub fn opacity(mut self, opacity: TextureId) -> Self {
        self.opacity = Some(ShaderInput::Texture(opacity));
        self
    }
    pub fn build(self) -> Material {
        Material {
            shader_model: self.shader_model,
            blend_mode: self.blend_mode,
            color: self.color,
            specular: self.specular,
            normal: self.normal,
            metallic: self.metallic,
            roughness: self.roughness,
            emissive: self.emissive,
            opacity: self.opacity,
        }
    }
}

struct MaterialUniform {
    color: [f32; 4],
    specular: [f32; 4],
    normal: [f32; 4],
    metallic: [f32; 4],
    roughness: [f32; 4],
    emissive: [f32; 4],
    opacity: [f32; 4],
}

impl MaterialUniform {
    pub fn new() -> Self {
        Self {
            color: [0.0; 4],
            specular: [0.0; 4],
            normal: [0.0; 4],
            metallic: [0.0; 4],
            roughness: [0.0; 4],
            emissive: [0.0; 4],
            opacity: [0.0; 4],
        }
    }

    pub fn from_material(material: &Material) -> Self {
        Self {
            color: get_color(&material.color),
            specular: get_color(&material.specular),
            normal: get_color(&material.normal),
            metallic: get_color(&material.metallic),
            roughness: get_color(&material.roughness),
            emissive: get_color(&material.emissive),
            opacity: get_color(&material.opacity),
        }
    }
}

pub struct ShaderProgram {
    pipeline: wgpu::RenderPipeline,
}

fn get_color(input: &Option<ShaderInput>) -> [f32; 4] {
    input.map_or([0.0; 4], |input| match input {
        ShaderInput::Color(color) => color.into(),
        ShaderInput::Scalar(scalar) => [scalar, scalar, scalar, scalar],
        ShaderInput::Texture(_) => [0.0; 4],
    })
}

impl ShaderProgram {
    fn get_material_block(material: &Material) -> String {
        let mut block = String::new();

        match material.shader_model {
            ShaderModel::Lit => {
                block.push_str("struct Material {");
                block.push_str(&Self::get_material_input(material, "color", material.color));
                block.push_str(&Self::get_material_input(
                    material,
                    "specular",
                    material.specular,
                ));
                block.push_str(&Self::get_material_input(
                    material,
                    "normal",
                    material.normal,
                ));
                block.push_str(&Self::get_material_input(
                    material,
                    "metallic",
                    material.metallic,
                ));
                block.push_str(&Self::get_material_input(
                    material,
                    "roughness",
                    material.roughness,
                ));
                block.push_str(&Self::get_material_input(
                    material,
                    "emissive",
                    material.emissive,
                ));
                block.push_str(&Self::get_material_input(
                    material,
                    "opacity",
                    material.opacity,
                ));
                block.push_str("};");
            }
            ShaderModel::Unlit => {
                block.push_str("struct Material {");
                block.push_str(&Self::get_material_input(material, "color", material.color));
                block.push_str(&Self::get_material_input(
                    material,
                    "opacity",
                    material.opacity,
                ));
                block.push_str("};");
            }
        }

        if block.len() < 2 {
            String::new()
        } else {
            block
        }
    }

    fn get_texture_block(material: &Material) -> String {
        let set = 0;
        let mut binding = 0;

        let block = match material.shader_model {
            ShaderModel::Lit => {
                let color =
                    Self::get_texture_input(material, "color", material.color, set, &mut binding);
                let specular = Self::get_texture_input(
                    material,
                    "specular",
                    material.specular,
                    set,
                    &mut binding,
                );
                let normal =
                    Self::get_texture_input(material, "normal", material.normal, set, &mut binding);
                let metallic = Self::get_texture_input(
                    material,
                    "metallic",
                    material.metallic,
                    set,
                    &mut binding,
                );
                let roughness = Self::get_texture_input(
                    material,
                    "roughness",
                    material.roughness,
                    set,
                    &mut binding,
                );
                let emissive = Self::get_texture_input(
                    material,
                    "emissive",
                    material.emissive,
                    set,
                    &mut binding,
                );
                let opacity = Self::get_texture_input(
                    material,
                    "opacity",
                    material.opacity,
                    set,
                    &mut binding,
                );

                format!(
                    "{} {} {} {} {} {} {}",
                    color, specular, normal, metallic, roughness, emissive, opacity
                )
            }
            ShaderModel::Unlit => {
                let color =
                    Self::get_texture_input(material, "color", material.color, set, &mut binding);
                let opacity = Self::get_texture_input(
                    material,
                    "opacity",
                    material.opacity,
                    set,
                    &mut binding,
                );

                format!("{} {}", color, opacity)
            }
        };

        if block.len() < 2 {
            String::new()
        } else {
            let sampler = format!(
                "@group({}) @binding({}) var Sampler: sampler;",
                set, binding
            );
            format!("{} {}", block, sampler)
        }
    }

    fn get_material_input(material: &Material, name: &str, input: Option<ShaderInput>) -> String {
        input.map_or(String::new(), |input| match input {
            ShaderInput::Texture(_) => String::from(""),
            ShaderInput::Color(_) | ShaderInput::Scalar(_) => format!("{}: vec4<f32>;", name),
        })
    }

    fn get_texture_input(
        material: &Material,
        name: &str,
        input: Option<ShaderInput>,
        set: u32,
        binding: &mut u32,
    ) -> String {
        input.map_or(String::new(), |input| match input {
            ShaderInput::Texture(_) => {
                let value = format!(
                    "@group({}) @binding({}) var {}: texture2d<f32>;",
                    set, binding, name
                );
                *binding += 1;
                value
            }
            ShaderInput::Color(_) | ShaderInput::Scalar(_) => String::from(""),
        })
    }
}
