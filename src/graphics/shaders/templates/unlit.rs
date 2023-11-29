use crate::graphics::shaders::layout::ShaderLayout;

pub fn unlit_shader_template(layout: &ShaderLayout) -> String {
    let opacity_var = if layout.is_opaque() {
        "1.0"
    } else {
        "opacity.x"
    };
    let texture_defs = layout.texture_binding_defs(1);
    let texture_vars = layout.texture_binding_vars();
    let material_vars = layout.material_attribute_vars("material");

    format!(
        r#"

    struct MaterialUniform {{
        color: vec4<f32>;
        opacity: vec4<f32>;
    }};

    @group(1) @binding(0)
    var<uniform> material: MaterialUniform;

    {texture_defs}

    struct FragmentInput {{
        @location(0) position: vec3<f32>,
        @location(1) normal: vec3<f32>,
        @location(2) tex_coords: vec2<f32>,
    }};

    @location(0) frag_color: vec4<f32>;
 
    fn main(input: FragmentInput) {{
        {texture_vars}
        {material_vars}

        frag_color = vec4<f32>(color.rgb, {opacity_var});
    }}
    "#,
        texture_defs = texture_defs,
        texture_vars = texture_vars,
        material_vars = material_vars,
        opacity_var = opacity_var
    )
}
