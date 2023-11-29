use crate::graphics::shaders::layout::ShaderLayout;

pub fn forward_shader_template(layout: &ShaderLayout, max_lights: u32) -> String {
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
        specular: vec4<f32>;
        normal: vec4<f32>;
        metallic: vec4<f32>;
        roughness: vec4<f32>;
        emissive: vec4<f32>;
        opacity: vec4<f32>;
    }};

    struct Light {{
        position: vec3<f32>;
        direction: vec3<f32>;
        color: vec3<f32>;
        intensity: f32;
        range: f32;
        kind: u32;
        spot_angle: f32;
    }};

    struct LightsBuffer {{
        lights: array<Light, {max_lights}>;
    }};

    @group(0) @binding(1)
    var<uniform> lights_buffer: LightsBuffer;
    
    @group(1) @binding(0)
    var<uniform> material: MaterialUniform;

    {texture_defs}
    
    struct FragmentInput {{
        @location(0) position: vec3<f32>,
        @location(1) normal: vec3<f32>,
        @location(2) tex_coords: vec2<f32>,
    }};

    @location(0) var<out> fragColor: vec4<f32>;

    fn calculate_light(position: vec3<f32>, normal: vec3<f32>, material: MaterialUniform, light: Light) -> vec3<f32> {{
        if light.kind == 0u {{
            return calculate_point_light(position, normal, material, light);
        }} else if light.kind == 1u {{
            return calculate_spot_light(position, normal, material, light);
        }} else if light.kind == 2u {{
            return calculate_dir_light(position, normal, material, light);
        }} else {{
            return vec3<f32>(0.0, 0.0, 0.0);
        }}
    }}

    fn calculate_lights(position: vec3<f32>, normal: vec3<f32>, material: MaterialUniform, lights: array<Light, {max_lights}>) -> vec3<f32> {{
        let mut color = vec3<f32>(0.0, 0.0, 0.0);

        for light in lights {{
            color += calculate_light(light, normal, position);
        }}

        color
    }}

    fn calculate_point_light(
        fragCoord: vec3<f32>,
        fragNormal: vec3<f32>,
        material: MaterialUniform,
        light: Light
    ) -> vec3<f32> {{
        var lightDirection = normalize(light.position - fragCoord);
        var lightColor = light.color;
        var attenuation = 1.0;
    
        // Point light
        var lightToFragment = light.position - fragCoord;
        var lightDistance = length(lightToFragment);
        attenuation = 1.0 / (1.0 + 0.05 * lightDistance + 0.007 * lightDistance * lightDistance);
    
        // Calculate the diffuse reflection
        var diffuseFactor = max(dot(fragNormal, lightDirection), 0.0);
        var diffuse = material.diffuseColor * lightColor * diffuseFactor * attenuation;
    
        // Calculate the specular reflection (Cook-Torrance model with roughness and metallic)
        var viewDirection = normalize(-fragCoord); // Assuming camera is at the origin
        var halfVector = normalize(lightDirection + viewDirection);
        var NdotH = max(dot(fragNormal, halfVector), 0.0);
        var D = (2.0 * NdotH * NdotH - 1.0) / (material.roughness * material.roughness * NdotH * NdotH);
        var G = (2.0 * NdotH) / (dot(fragNormal, viewDirection) + dot(fragNormal, lightDirection));
        var F = mix(0.04, 1.0, material.metallic);
        var specular = ((D * G * F) / (4.0 * dot(fragNormal, viewDirection) * dot(fragNormal, lightDirection))) * material.specularColor * lightColor * attenuation;
    
        // Calculate the final color
        return diffuse + specular;
    }}

    fn calculate_dir_light(
        fragCoord: vec3<f32>,
        fragNormal: vec3<f32>,
        material: MaterialUniform,
        light: Light
    ) -> vec3<f32> {{
        var lightDirection = -normalize(light.direction);
        var lightColor = light.color;
    
        // Directional light
        var attenuation = 1.0; // No attenuation for directional lights
    
        // Calculate the diffuse reflection
        var diffuseFactor = max(dot(fragNormal, lightDirection), 0.0);
        var diffuse = material.diffuseColor * lightColor * diffuseFactor * attenuation;
    
        // Calculate the specular reflection (Cook-Torrance model with roughness and metallic)
        var viewDirection = normalize(-fragCoord); // Assuming camera is at the origin
        var halfVector = normalize(lightDirection + viewDirection);
        var NdotH = max(dot(fragNormal, halfVector), 0.0);
        var D = (2.0 * NdotH * NdotH - 1.0) / (material.roughness * material.roughness * NdotH * NdotH);
        var G = (2.0 * NdotH) / (dot(fragNormal, viewDirection) + dot(fragNormal, lightDirection));
        var F = mix(0.04, 1.0, material.metallic);
        var specular = ((D * G * F) / (4.0 * dot(fragNormal, viewDirection) * dot(fragNormal, lightDirection))) * material.specularColor * lightColor * attenuation;
    
        // Calculate the final color
        return diffuse + specular;
    }}
    

    fn calculate_spot_light(
        fragCoord: vec3<f32>,
        fragNormal: vec3<f32>,
        material: MaterialUniform,
        light: Light
    ) -> vec3<f32> {{
        var lightDirection = normalize(light.position - fragCoord);
        var lightColor = light.color;
        var attenuation = 1.0;
    
        // Spot light
        var lightToFragment = normalize(light.position - fragCoord);
        var lightDistance = length(lightToFragment);
        var spotAngleCos = dot(-lightToFragment, normalize(light.direction));
    
        // Check if the fragment is inside the spot cone
        if (spotAngleCos > 0.85) {{
            // Apply attenuation based on distance and spot angle
            attenuation = 1.0 / (1.0 +  0.05 * lightDistance + 0.007 * lightDistance * lightDistance);
        }} else if (spotAngleCos > 0.9) {{
            // Apply penumbra attenuation if inside the inner cone
            var smoothstepFactor = smoothstep(0.9, 0.85, spotAngleCos);
            attenuation = 1.0 / (1.0 +  0.05 * lightDistance + 0.007 * lightDistance * lightDistance);
            attenuation = attenuation * smoothstepFactor;
        }} else {{
            // Outside of the spot cone
            attenuation = 0.0;
        }}
    
        // Calculate the diffuse reflection
        var diffuseFactor = max(dot(fragNormal, lightDirection), 0.0);
        var diffuse = material.diffuseColor * lightColor * diffuseFactor * attenuation;
    
        // Calculate the specular reflection (Cook-Torrance model with roughness and metallic)
        var viewDirection = normalize(-fragCoord); // Assuming camera is at the origin
        var halfVector = normalize(lightDirection + viewDirection);
        var NdotH = max(dot(fragNormal, halfVector), 0.0);
        var D = (2.0 * NdotH * NdotH - 1.0) / (material.roughness * material.roughness * NdotH * NdotH);
        var G = (2.0 * NdotH) / (dot(fragNormal, viewDirection) + dot(fragNormal, lightDirection));
        var F = mix(0.04, 1.0, material.metallic);
        var specular = ((D * G * F) / (4.0 * dot(fragNormal, viewDirection) * dot(fragNormal, lightDirection))) * material.specularColor * lightColor * attenuation;
    
        // Calculate the final color
        return diffuse + specular;
    }}    

    fn main(input: FragmentInput) {{
        // Normalize the normal vector
        var normal : vec3<f32> = normalize(input.normal);

        // Get color from texture or uniform
        {texture_vars}
        {material_vars}

        var material_uniform: MaterialUniform = MaterialUniform(
            color,
            specular,
            normal,
            metallic,
            roughness,
            emissive,
            opacity
        );

        // Calculate the lights
        var finalColor: vec3<f32> = calculate_lights(input.position, normal, material_uniform, lights_buffer.lights);

        // Multiply the final color by the texture color
        finalColor = finalColor * color;

        // Set the output color
        fragColor = vec4<f32>(finalColor, {opacity_var});
    }}
    "#,
        texture_defs = texture_defs,
        texture_vars = texture_vars,
        material_vars = material_vars,
        opacity_var = opacity_var,
        max_lights = max_lights,
    )
}
