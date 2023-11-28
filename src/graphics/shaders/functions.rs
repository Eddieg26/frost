// pub const VERTEX_INPUT: &str = r#"
// struct VertexInput {
//     @location(0) position: vec3<f32>,
//     @location(1) normal: vec3<f32>,
//     @location(2) tex_coords: vec2<f32>,
// }
// "#;

// pub const VERTEX_OUTPUT: &str = r#"
// struct VertexOutput {
//     @builtin(position) clip_position: vec4<f32>,
//     @location(0) normal: vec3<f32>,
//     @location(1) tex_coords: vec2<f32>,
// }
// "#;

pub const FRAGMENT_INPUT: &str = r#"
struct FragmentInput {
    @location(0) normal: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}
"#;

pub const FRAGMENT_OUTPUT: &str = r#"
struct FragmentOutput {
    @location(0) color: vec4<f32>;
    @location(1) depth: f32;
}
"#;

pub const LIGHT_STRUCT: &str = r#"
struct Light {
    position: vec3<f32>,
    direction: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    range: f32,
    kind: LightKind,
    spot_angle: f32,
}
"#;

pub const HELPER_FUNCTIONS: &str = r#"
fn calculate_light(light: Light, normal: vec3<f32>, position: vec3<f32>) -> vec3<f32> {
    match light.kind {
        LightKind::Point => calculate_point_light(light, normal, position),
        LightKind::Directional => calculate_dir_light(light, normal, position),
        LightKind::Spot => calculate_spot_light(light, normal, position),
    }
}

fn calculate_lights(lights: [Light; 16], normal: vec3<f32>, position: vec3<f32>) -> vec3<f32> {
    let mut color = vec3<f32>(0.0, 0.0, 0.0);

    for light in lights {
        color += calculate_light(light, normal, position);
    }

    color
}

fn calculate_point_light(light: Light, normal: vec3<f32>, position: vec3<f32>) -> vec3<f32> {
    let light_dir = light.position - position;
    let light_distance = length(light_dir);
    let light_dir = normalize(light_dir);
    let light_intensity = light.intensity / (light_distance * light_distance);
    let light_color = light.color * light_intensity;
    let light_attenuation = saturate(1.0 - light_distance / light.range);

    let diffuse = saturate(dot(normal, light_dir));
    let specular = 0.0;

    light_color * light_attenuation * (diffuse + specular)
}

fn calculate_dir_light(light: Light, normal: vec3<f32>, position: vec3<f32>) -> vec3<f32> {
    let light_dir = -light.direction;
    let light_intensity = light.intensity;
    let light_color = light.color * light_intensity;

    let diffuse = saturate(dot(normal, light_dir));
    let specular = 0.0;

    light_color * (diffuse + specular)
}

fn calculate_spot_light(light: Light, normal: vec3<f32>, position: vec3<f32>) -> vec3<f32> {
    let light_dir = light.position - position;
    let light_distance = length(light_dir);
    let light_dir = normalize(light_dir);
    let light_intensity = light.intensity / (light_distance * light_distance);
    let light_color = light.color * light_intensity;
    let light_attenuation = saturate(1.0 - light_distance / light.range);

    let diffuse = saturate(dot(normal, light_dir));
    let specular = 0.0;

    let spot_angle = dot(light_dir, -light.direction);
    let spot_angle = saturate((spot_angle - light.spot_angle) / (1.0 - light.spot_angle));

    light_color * light_attenuation * (diffuse + specular) * spot_angle
}
"#;
