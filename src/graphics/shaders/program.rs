use crate::graphics::{
    material::{Material, ShaderInput},
    shaders::functions::{FRAGMENT_INPUT, FRAGMENT_OUTPUT, HELPER_FUNCTIONS},
    texture::Texture,
    Graphics,
};

pub struct MaterialProps {
    props: Vec<String>,
}

impl MaterialProps {
    fn from_material(material: &Material) -> Self {
        let mut props = vec![];

        Self::get_input_prop(&material.color(), &mut props);
        Self::get_input_prop(&material.specular(), &mut props);
        Self::get_input_prop(&material.normal(), &mut props);
        Self::get_input_prop(&material.metallic(), &mut props);
        Self::get_input_prop(&material.roughness(), &mut props);
        Self::get_input_prop(&material.emissive(), &mut props);
        Self::get_input_prop(&material.opacity(), &mut props);

        Self { props }
    }

    fn get_input_prop(input: &Option<&ShaderInput>, props: &mut Vec<String>) {
        match input {
            Some(ShaderInput::Color(_)) | Some(ShaderInput::Scalar(_)) => {
                props.push("color".to_string())
            }
            _ => {}
        }
    }

    fn create_shader_block(&self) -> String {
        let mut block = String::new();

        for prop in &self.props {
            block.push_str(&format!(
                "var {} : vec3<f32> = material.{}.xyz;\n",
                prop, prop
            ));
        }

        block
    }
}

pub struct TextureBindGroup {
    group: u32,
    bindings: Vec<TextureBinding>,
    layout: wgpu::BindGroupLayout,
}

pub struct MaterialBindGroup {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl TextureBindGroup {
    fn new(group: u32, bindings: Vec<TextureBinding>, graphics: &Graphics) -> Self {
        Self {
            group,
            layout: Self::create_bind_group_layout(&bindings, graphics),
            bindings,
        }
    }

    fn from_material(group: u32, material: &Material, graphics: &Graphics) -> Self {
        let mut bindings = vec![];

        if material.color().is_some() {
            bindings.push(TextureBinding::new("color"));
        }

        if material.specular().is_some() {
            bindings.push(TextureBinding::new("specular"));
        }

        if material.normal().is_some() {
            bindings.push(TextureBinding::new("normal"));
        }

        if material.metallic().is_some() {
            bindings.push(TextureBinding::new("metallic"));
        }

        if material.roughness().is_some() {
            bindings.push(TextureBinding::new("roughness"));
        }

        if material.emissive().is_some() {
            bindings.push(TextureBinding::new("emissive"));
        }

        if material.opacity().is_some() {
            bindings.push(TextureBinding::new("opacity"));
        }

        Self::new(group, bindings, graphics)
    }

    fn create_bind_group_layout(
        bindings: &Vec<TextureBinding>,
        graphics: &Graphics,
    ) -> wgpu::BindGroupLayout {
        let mut entries = Vec::new();

        entries.push(wgpu::BindGroupLayoutEntry {
            binding: entries.len() as u32,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        for _ in bindings {
            entries.push(wgpu::BindGroupLayoutEntry {
                binding: entries.len() as u32,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            });

            entries.push(wgpu::BindGroupLayoutEntry {
                binding: entries.len() as u32,
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
                    label: Some("texture_bind_group_layout"),
                    entries: &entries,
                });

        layout
    }

    fn create_bind_group(&self, graphics: &Graphics, material: &Material) -> MaterialBindGroup {
        let mut entries = Vec::new();

        let uniform = MaterialUniform::from_material(material);
        let buffer = graphics.create_uniform_buffer(bytemuck::bytes_of(&uniform));

        entries.push(wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        });

        for (idx, (texture, sampler)) in self
            .get_material_textures(graphics, material)
            .iter()
            .enumerate()
        {
            entries.push(wgpu::BindGroupEntry {
                binding: (idx * 2 + 1) as u32,
                resource: wgpu::BindingResource::TextureView(texture.view()),
            });

            entries.push(wgpu::BindGroupEntry {
                binding: (idx * 2 + 2) as u32,
                resource: wgpu::BindingResource::Sampler(sampler),
            });
        }

        let bind_group = graphics
            .gpu()
            .device()
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.layout,
                entries: &entries,
            });

        MaterialBindGroup { buffer, bind_group }
    }

    fn get_material_textures<'a>(
        &'a self,
        graphics: &'a Graphics,
        material: &'a Material,
    ) -> Vec<(&'a dyn Texture, &'a wgpu::Sampler)> {
        self.bindings
            .iter()
            .filter_map(|binding| {
                let texture = (binding.get_texture)(graphics, material)?;
                let sampler = texture.sampler();

                Some((texture, sampler))
            })
            .collect::<Vec<_>>()
    }

    fn create_shader_block(&self) -> String {
        let mut block = String::new();
        let mut idx = 0;

        for binding in &self.bindings {
            block.push_str(&format!(
                "@group({}) @binding({}) var {}_tex: texture2d<f32>;\n",
                self.group, idx, binding.name
            ));

            idx += 1;

            let sampler_name = format!("{}_sampler", binding.name);
            block.push_str(&format!(
                "@group({}) @binding({}) var {}: sampler;\n",
                self.group, binding.name, sampler_name
            ));

            idx += 1;
        }

        block
    }
}

pub struct TextureBinding {
    name: String,
    get_texture: Box<dyn for<'a> Fn(&'a Graphics, &'a Material) -> Option<&'a dyn Texture>>,
}

impl TextureBinding {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            get_texture: Self::create_get_texture(name),
        }
    }

    fn create_get_texture(
        name: &str,
    ) -> Box<dyn for<'a> Fn(&'a Graphics, &'a Material) -> Option<&'a dyn Texture>> {
        match name {
            "color" => Box::new(|graphics, material| {
                Self::get_shader_input_texture(graphics, material.color())
            }),
            "specular" => Box::new(|graphics, material| {
                Self::get_shader_input_texture(graphics, material.specular())
            }),
            "normal" => Box::new(|graphics, material| {
                Self::get_shader_input_texture(graphics, material.normal())
            }),
            "metallic" => Box::new(|graphics, material| {
                Self::get_shader_input_texture(graphics, material.metallic())
            }),
            "roughness" => Box::new(|graphics, material| {
                Self::get_shader_input_texture(graphics, material.roughness())
            }),
            "emissive" => Box::new(|graphics, material| {
                Self::get_shader_input_texture(graphics, material.emissive())
            }),
            "opacity" => Box::new(|graphics, material| {
                Self::get_shader_input_texture(graphics, material.opacity())
            }),
            _ => Box::new(|_, _| None),
        }
    }

    fn get_shader_input_texture<'a>(
        graphics: &'a Graphics,
        input: Option<&'a ShaderInput>,
    ) -> Option<&'a dyn Texture> {
        input.map_or(None, |input| match input {
            ShaderInput::Texture(id) => graphics.dyn_texture(&id),
            _ => None,
        })
    }

    fn create_shader_block(&self) -> String {
        format!(
            "var {} : vec3<f32> = textureSample({}_tex, {}_sampler, input.tex_coords).xyz;\n",
            self.name, self.name, self.name
        )
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct MaterialUniform {
    color: [f32; 4],
    specular: [f32; 4],
    normal: [f32; 4],
    metallic: [f32; 4],
    roughness: [f32; 4],
    emissive: [f32; 4],
    opacity: [f32; 4],
}

impl MaterialUniform {
    pub fn new() -> MaterialUniform {
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

    fn from_material(material: &Material) -> MaterialUniform {
        let mut uniform = Self::new();

        uniform.color = Self::get_input_color(material.color());
        uniform.specular = Self::get_input_color(material.specular());
        uniform.normal = Self::get_input_color(material.normal());
        uniform.metallic = Self::get_input_color(material.metallic());
        uniform.roughness = Self::get_input_color(material.roughness());
        uniform.emissive = Self::get_input_color(material.emissive());
        uniform.opacity = Self::get_input_color(material.opacity());

        uniform
    }

    fn get_input_color(color: Option<&ShaderInput>) -> [f32; 4] {
        color.map_or([0.0; 4], |color| match color {
            ShaderInput::Color(color) => color.into(),
            ShaderInput::Scalar(scalar) => [*scalar, *scalar, *scalar, 1.0],
            _ => [0.0; 4],
        })
    }

    fn create_shader_block() -> String {
        let mut block = String::new();

        block.push_str(
            r#"
            struct MaterialUniform {
                color: vec4<f32>;
                specular: vec4<f32>;
                normal: vec4<f32>;
                metallic: vec4<f32>;
                roughness: vec4<f32>;
                emissive: vec4<f32>;
                opacity: vec4<f32>;
            };
        "#,
        );

        block
    }
}

const BIND_GROUP: u32 = 0;

pub struct ShaderProgram {
    texture_bind_group: TextureBindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl ShaderProgram {
    pub fn new(material: &Material, graphics: &Graphics) -> Self {
        let texture_bind_group = TextureBindGroup::from_material(BIND_GROUP, material, graphics);
        let material_props = MaterialProps::from_material(material);

        let main_block = Self::create_main_block(&texture_bind_group, &material_props);
        let texture_block = texture_bind_group.create_shader_block();
        let material_block = MaterialUniform::create_shader_block();

        let shader_str = format!(
            "{} {} {} {} {} {}",
            HELPER_FUNCTIONS,
            FRAGMENT_INPUT,
            FRAGMENT_OUTPUT,
            material_block,
            texture_block,
            main_block
        );

        let fragment_shader =
            graphics
                .gpu()
                .device()
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(shader_str)),
                });

        todo!()
    }

    fn create_main_block(
        texture_group: &TextureBindGroup,
        material_props: &MaterialProps,
    ) -> String {
        let texture_block = {
            let mut block = String::new();

            for binding in &texture_group.bindings {
                block.push_str(&binding.create_shader_block());
            }

            block
        };
        let material_block = material_props.create_shader_block();

        let mut main_block = String::new();
        main_block.push_str(&format!(
            r#"
            [[stage(fragment)]]
            fn main(input: FragmentInput) -> void {{
                // Normalize the normal vector
                var normal : vec3<f32> = normalize(input.normal);

                // Get color from texture or uniform
                {}
                {}

                // Calculate the lights
                var finalColor: vec3<f32> = calculate_lights(lights, normal, input.position);

                // Multiply the final color by the texture color
                finalColor = finalColor * color;

                // Set the output color
                fragColor = vec4<f32>(finalColor, opacity.x);
            }}
            "#,
            texture_block, material_block
        ));

        main_block
    }
}
