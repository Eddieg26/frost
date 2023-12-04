use super::layout::ShaderLayout;

pub struct UnlitShaderTemplate;

impl UnlitShaderTemplate {
    pub fn create_shader(device: &wgpu::Device, layout: &ShaderLayout) -> wgpu::ShaderModule {
        let shader = unlit_shader_template(layout);
        let module = wgpu::ShaderSource::Wgsl(shader.into());

        device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("unlit shader"),
            source: module,
        })
    }
}

fn unlit_shader_template(layout: &ShaderLayout) -> String {
    let texture_defs = layout.texture_binding_defs(2);
    let sampler_defs = layout.sampler_binding_defs(2);
    let texture_vars = layout.texture_binding_vars();
    let material_vars = layout.material_attribute_vars("material");

    format!(
        r#"
        struct VertexInput {{
            @location(0) position: vec3<f32>,
            @location(1) normal: vec3<f32>,
            @location(2) tex_coords: vec2<f32>,
        }}

        struct VertexOutput {{
            @builtin(position) position: vec4<f32>,
            @location(0) tex_coords: vec2<f32>,
        }}

        struct Camera {{
            view: mat4x4<f32>,
            projection: mat4x4<f32>,
        }}

        struct Object {{
            model: mat4x4<f32>
        }}

        @group(0) @binding(0)
        var<uniform> camera: Camera;

        @group(1) @binding(0)
        var<uniform> object: Object;

        @vertex
        fn vs_main(input: VertexInput) -> VertexOutput {{
            var out: VertexOutput;
            out.position = vec4<f32>(input.position.xyz, 1.0);
            out.tex_coords = input.tex_coords;
            return out;
        }}

        struct Material {{
            color: vec4<f32>;
            opacity: vec4<f32>;
        }}

        struct Standard {{
            color: vec4<f32>;
            opacity: f32;
        }}

        @group(2) @binding(0)
        var<uniform> material: Material;

        {texture_defs}
        {sampler_defs}

        @fragment
        fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {{
            {texture_vars}
            {material_vars}

            let standard: Standard;
            standard.color = color;
            standard.opacity = opacity.x;

            return vec4<f32>(standard.color.rgb, standard.opacity);
        }}
    "#,
        texture_defs = texture_defs,
        texture_vars = texture_vars,
        material_vars = material_vars,
    )
}
