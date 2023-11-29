use crate::graphics::material::{BlendMode, Material, ShaderInput, ShaderModel};

pub struct MaterialBindGroup {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl MaterialBindGroup {
    pub fn new(buffer: wgpu::Buffer, bind_group: wgpu::BindGroup) -> MaterialBindGroup {
        Self { buffer, bind_group }
    }
}

pub struct TextureBinding {
    pub binding: u32,
    pub name: String,
}

impl TextureBinding {
    pub fn new(binding: u32, name: String) -> Self {
        Self { binding, name }
    }

    pub fn get_binding_def(&self, group: u32) -> String {
        let sampler_binding = self.binding + 1;
        let sampler_name = format!("{}_sampler", self.name);

        format!(
            r#"
            @group({group}) @binding({texture_binding})
            var<uniform> {texture}_tex: texture_2d<f32>;

            @group({group}) @binding({sampler_binding})
            var<uniform> {sampler}_sampler: texture_2d<f32>;
        "#,
            group = group,
            texture = self.name,
            texture_binding = self.binding,
            sampler_binding = sampler_binding,
            sampler = sampler_name
        )
    }

    pub fn get_binding_var(&self) -> String {
        format!(
            r#"
            let {texture} = textureSample({texture}_tex, {sampler}_sampler, input.tex_coords);
        "#,
            texture = self.name,
            sampler = format!("{}_sampler", self.name)
        )
    }
}

pub struct MaterialAttribute {
    pub name: String,
}

impl MaterialAttribute {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn get_var(&self, buffer_name: &str) -> String {
        format!(
            r#"
            let {name} = {buffer_name}.{name};
        "#,
            name = self.name,
            buffer_name = buffer_name
        )
    }
}

pub struct ShaderLayout {
    texture_bindings: Vec<TextureBinding>,
    material_attributes: Vec<MaterialAttribute>,
    blend_mode: BlendMode,
}

impl ShaderLayout {
    fn new(blend_mode: BlendMode) -> ShaderLayout {
        ShaderLayout {
            texture_bindings: Vec::new(),
            material_attributes: Vec::new(),
            blend_mode,
        }
    }

    pub fn from_material(material: &Material) -> ShaderLayout {
        let mut layout = ShaderLayout::new(material.blend_mode());
        layout.add_input("color", material.color());
        layout.add_input("opacity", material.opacity());
        if material.shader_model() == ShaderModel::Lit {
            layout.add_input("specular", material.specular());
            layout.add_input("normal", material.normal());
            layout.add_input("metallic", material.metallic());
            layout.add_input("roughness", material.roughness());
            layout.add_input("emissive", material.emissive());
        }

        layout
    }

    pub fn blend_mode(&self) -> BlendMode {
        self.blend_mode
    }

    pub fn is_opaque(&self) -> bool {
        self.blend_mode == BlendMode::Opaque
    }

    pub fn texture_bindings(&self) -> &Vec<TextureBinding> {
        &self.texture_bindings
    }

    pub fn material_attributes(&self) -> &Vec<MaterialAttribute> {
        &self.material_attributes
    }

    fn add_input(&mut self, name: &str, input: Option<&ShaderInput>) {
        match input {
            Some(ShaderInput::Texture(_)) => self.texture_bindings.push(TextureBinding::new(
                self.texture_bindings.len() as u32,
                name.to_string(),
            )),
            Some(ShaderInput::Color(_)) | Some(ShaderInput::Scalar(_)) | None => self
                .material_attributes
                .push(MaterialAttribute::new(name.to_string())),
        }
    }

    pub fn texture_binding_defs(&self, group: u32) -> String {
        self.texture_bindings
            .iter()
            .map(|binding| binding.get_binding_def(group))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn texture_binding_vars(&self) -> String {
        self.texture_bindings
            .iter()
            .map(|binding| binding.get_binding_var())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn material_attribute_vars(&self, buffer_name: &str) -> String {
        self.material_attributes
            .iter()
            .map(|attr| attr.get_var(buffer_name))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
