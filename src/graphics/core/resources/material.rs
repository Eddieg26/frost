use crate::graphics::{color::Color, TextureId};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ShaderModel {
    Lit,
    Unlit,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlendMode {
    Opaque,
    Translucent,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MaterialInput {
    Texture(TextureId),
    Color(Color),
    Scalar(f32),
}

pub struct Material {
    shader_model: ShaderModel,
    blend_mode: BlendMode,
    color: Option<MaterialInput>,
    normal: Option<MaterialInput>,
    specular: Option<MaterialInput>,
    metallic: Option<MaterialInput>,
    roughness: Option<MaterialInput>,
    emissive: Option<MaterialInput>,
    opacity: Option<MaterialInput>,
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

    pub fn color(&self) -> Option<&MaterialInput> {
        self.color.as_ref()
    }

    pub fn specular(&self) -> Option<&MaterialInput> {
        self.specular.as_ref()
    }

    pub fn normal(&self) -> Option<&MaterialInput> {
        self.normal.as_ref()
    }

    pub fn metallic(&self) -> Option<&MaterialInput> {
        self.metallic.as_ref()
    }

    pub fn roughness(&self) -> Option<&MaterialInput> {
        self.roughness.as_ref()
    }

    pub fn emissive(&self) -> Option<&MaterialInput> {
        self.emissive.as_ref()
    }

    pub fn opacity(&self) -> Option<&MaterialInput> {
        self.opacity.as_ref()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MaterialInfo {
    shader_model: ShaderModel,
    blend_mode: BlendMode,
    color: Option<MaterialInput>,
    specular: Option<MaterialInput>,
    normal: Option<MaterialInput>,
    metallic: Option<MaterialInput>,
    roughness: Option<MaterialInput>,
    emissive: Option<MaterialInput>,
    opacity: Option<MaterialInput>,
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
        self.color = Some(MaterialInput::Color(color));
        self
    }

    pub fn specular(mut self, specular: TextureId) -> Self {
        self.specular = Some(MaterialInput::Texture(specular));
        self
    }

    pub fn normal(mut self, normal: TextureId) -> Self {
        self.normal = Some(MaterialInput::Texture(normal));
        self
    }

    pub fn metallic(mut self, metallic: TextureId) -> Self {
        self.metallic = Some(MaterialInput::Texture(metallic));
        self
    }

    pub fn roughness(mut self, roughness: TextureId) -> Self {
        self.roughness = Some(MaterialInput::Texture(roughness));
        self
    }

    pub fn emissive(mut self, emissive: TextureId) -> Self {
        self.emissive = Some(MaterialInput::Texture(emissive));
        self
    }

    pub fn opacity(mut self, opacity: TextureId) -> Self {
        self.opacity = Some(MaterialInput::Texture(opacity));
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
