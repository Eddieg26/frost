

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

struct Camera {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
}

struct Object {
    model: mat4x4<f32>
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> object: Object;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(input.position.xyz, 1.0);
    out.tex_coords = input.tex_coords;
    return out;
}

struct Material {
    color: vec4<f32>;
    opacity: vec4<f32>;
}

struct Standard {
    color: vec4<f32>;
    opacity: f32;
}

@group(2) @binding(0)
var<uniform> material: Material;

@group(2) @binding(1)
var color_texture: texture_2d<f32>;

@group(2) @binding(2)
var opacity_texture: texture_2d<f32>;

@group(2) @binding(3)
var color_sampler: sampler;

@group(2) @binding(4)
var opacity_sampler: sampler;

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(color_texture, color_sampler, input.tex_coords);
    // let color = vec4<f32>(material.color.xyz, 1.0);

    let opacity = textureSample(opacity_texture, opacity_sampler, input.tex_coords).x;
    // let opacity = material.opacity.x;
    // let opacity = 1.0;

    let standard: Standard;
    standard.color = color;
    standard.opacity = opacity;

    return vec4<f32>(standard.color.rgb, standard.opacity);
}