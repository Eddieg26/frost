use crate::graphics::{color::Color, TextureId};
use std::hash::Hash;
use std::hash::Hasher;

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

impl Into<wgpu::BlendState> for BlendMode {
    fn into(self) -> wgpu::BlendState {
        match self {
            Self::Opaque => wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                },
                alpha: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::Zero,
                },
            },
            Self::Translucent => wgpu::BlendState {
                color: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                },
                alpha: wgpu::BlendComponent {
                    operation: wgpu::BlendOperation::Add,
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                },
            },
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Material {
    shader_model: ShaderModel,
    blend_mode: BlendMode,
    color: ShaderInput,
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

    pub fn color(&self) -> &ShaderInput {
        &self.color
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

    pub fn get_input_color(input: &Option<&ShaderInput>, default: [f32; 4]) -> [f32; 4] {
        match input {
            Some(ShaderInput::Color(color)) => color.into(),
            Some(ShaderInput::Scalar(scaler)) => [*scaler, *scaler, *scaler, 1.0],
            _ => default,
        }
    }
}

impl Hash for Material {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.shader_model.hash(state);
        self.blend_mode.hash(state);
        self.color.hash(state);
        self.opacity.hash(state);
        if self.shader_model == ShaderModel::Lit {
            self.specular.hash(state);
            self.normal.hash(state);
            self.metallic.hash(state);
            self.roughness.hash(state);
            self.emissive.hash(state);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MaterialInfo {
    shader_model: ShaderModel,
    blend_mode: BlendMode,
    color: ShaderInput,
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
            color: ShaderInput::Color(Color::white()),
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

    pub fn color(mut self, color: ShaderInput) -> Self {
        self.color = color;
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
    pub fn build(&self) -> Material {
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
