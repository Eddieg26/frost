use crate::graphics::{
    material::{BlendMode, Material, ShaderInput, ShaderModel},
    texture::Texture,
    Graphics,
};

pub struct ShaderLayout {
    texture_bindings: Vec<TextureBinding>,
    material_attributes: Vec<MaterialAttribute>,
    blend_mode: BlendMode,
    model: ShaderModel,
}

impl ShaderLayout {
    fn new(model: ShaderModel, blend_mode: BlendMode) -> ShaderLayout {
        ShaderLayout {
            texture_bindings: Vec::new(),
            material_attributes: Vec::new(),
            blend_mode,
            model,
        }
    }

    pub fn from_material(material: &Material) -> ShaderLayout {
        let mut layout = ShaderLayout::new(material.shader_model(), material.blend_mode());
        layout.add_input("color", Some(material.color()));
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

    pub fn shader_model(&self) -> ShaderModel {
        self.model
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

    pub fn sampler_binding_defs(&self, group: u32) -> String {
        self.texture_bindings
            .iter()
            .map(|binding| binding.get_sampler_def(group))
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

    pub fn bind_group_layout_entries(&self) -> Vec<wgpu::BindGroupLayoutEntry> {
        let mut entries = vec![wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }];

        for _ in &self.texture_bindings {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: entries.len() as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            });
        }

        for _ in &self.texture_bindings {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: entries.len() as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            });
        }

        entries
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
        format!(
            r#"
            @group({group}) @binding({texture_binding})
            var<uniform> {texture}_tex: texture_2d<f32>;
        "#,
            group = group,
            texture = self.name,
            texture_binding = self.binding,
        )
    }

    pub fn get_sampler_def(&self, binding: u32) -> String {
        let sampler_name = format!("{}_sampler", self.name);

        format!(
            r#"
            @group(1) @binding({sampler_binding})
            var<uniform> {sampler}: sampler;
        "#,
            sampler_binding = binding,
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

type TextureGetters = Vec<for<'a> fn(&'a Graphics, &'a Material) -> Option<&'a dyn Texture>>;

pub struct MaterialBindGroupLayout {
    layout: wgpu::BindGroupLayout,
    getters: TextureGetters,
    model: ShaderModel,
}

impl MaterialBindGroupLayout {
    pub fn from_material(
        layout: wgpu::BindGroupLayout,
        material: &Material,
    ) -> MaterialBindGroupLayout {
        let mut getters: TextureGetters = vec![];

        getters.push(MaterialBindGroupLayout::get_color_texture);

        if material.shader_model() == ShaderModel::Lit {
            getters.push(MaterialBindGroupLayout::get_specular_texture);
            getters.push(MaterialBindGroupLayout::get_normal_texture);
            getters.push(MaterialBindGroupLayout::get_metallic_texture);
            getters.push(MaterialBindGroupLayout::get_roughness_texture);
            getters.push(MaterialBindGroupLayout::get_emissive_texture);
        }

        if material.blend_mode() == BlendMode::Translucent {
            getters.push(MaterialBindGroupLayout::get_opacity_texture);
        }

        MaterialBindGroupLayout {
            layout,
            getters,
            model: material.shader_model(),
        }
    }

    pub fn create_bind_group(&self, graphics: &Graphics, material: &Material) -> wgpu::BindGroup {
        let buffer = match self.model {
            ShaderModel::Lit => graphics.shader_resources().lit_material(),
            ShaderModel::Unlit => graphics.shader_resources().unlit_material(),
        };

        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                offset: 0,
                size: None,
            }),
        }];

        for texture in &self.getters {
            if let Some(texture) = texture(graphics, material) {
                entries.push(wgpu::BindGroupEntry {
                    binding: entries.len() as u32,
                    resource: wgpu::BindingResource::TextureView(texture.view()),
                });
            }
        }

        for texture in &self.getters {
            if let Some(texture) = texture(graphics, material) {
                entries.push(wgpu::BindGroupEntry {
                    binding: entries.len() as u32,
                    resource: wgpu::BindingResource::Sampler(texture.sampler()),
                });
            }
        }

        graphics
            .gpu()
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("material_bind_group"),
                layout: &self.layout,
                entries: &entries,
            })
    }

    fn get_color_texture<'a>(
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Option<&'a dyn Texture> {
        match material.color() {
            ShaderInput::Texture(id) => graphics.dyn_texture(id),
            _ => None,
        }
    }

    fn get_specular_texture<'a>(
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Option<&'a dyn Texture> {
        match material.specular() {
            Some(ShaderInput::Texture(id)) => graphics.dyn_texture(id),
            _ => None,
        }
    }

    fn get_normal_texture<'a>(
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Option<&'a dyn Texture> {
        match material.normal() {
            Some(ShaderInput::Texture(id)) => graphics.dyn_texture(id),
            _ => None,
        }
    }

    fn get_metallic_texture<'a>(
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Option<&'a dyn Texture> {
        match material.metallic() {
            Some(ShaderInput::Texture(id)) => graphics.dyn_texture(id),
            _ => None,
        }
    }

    fn get_roughness_texture<'a>(
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Option<&'a dyn Texture> {
        match material.roughness() {
            Some(ShaderInput::Texture(id)) => graphics.dyn_texture(id),
            _ => None,
        }
    }

    fn get_emissive_texture<'a>(
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Option<&'a dyn Texture> {
        match material.emissive() {
            Some(ShaderInput::Texture(id)) => graphics.dyn_texture(id),
            _ => None,
        }
    }

    fn get_opacity_texture<'a>(
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Option<&'a dyn Texture> {
        match material.opacity() {
            Some(ShaderInput::Texture(id)) => graphics.dyn_texture(id),
            _ => None,
        }
    }
}
