struct ShaderData {
    player_position: vec2<f32>,
    player_direction: vec2<f32>,
    goal_position: vec2<f32>
}

@group(1) @binding(0)
var<uniform> uniforms: ShaderData;

@group(1) @binding(1)
var light_texture: texture_2d<f32>;
@group(1) @binding(2)
var light_sampler: sampler;

@group(1) @binding(3)
var dark_texture: texture_2d<f32>;
@group(1) @binding(4)
var dark_sampler: sampler;


@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let light_color = textureSample(light_texture, light_sampler, uv);
    let dark_color = textureSample(dark_texture, dark_sampler, uv);

    var final_color = light_color;
    // Calculate the distance from the player and the goal to the current pixel
    let player_distance = distance(uniforms.player_position, uv);
    let goal_distance = distance(uniforms.goal_position, uv);
    
    let threshold: f32 = 200.0;
    // Check if we are in the circle range of the player or the goal
    if(player_distance > threshold || goal_distance > threshold) {
        final_color = light_color; // Make it lighter
    }
    else {
        final_color = dark_color; // Make it darker
    }
/*
    // Initial blending with 0.1 opacity for the dark dimension
    let final_color = mix(light_color, dark_color, 0.1);

    let light_intensity: f32 = 1.5;
    let dark_intensity: f32 = 0.5;
    let threshold: f32 = SOME_THRESHOLD; // Replace with your threshold

    // Check if we are in the circle range of the player or the goal
    if(player_distance > threshold && goal_distance > threshold) {
        // We are not in range, darken or lighten the texture depending on the dimension
        if (uniforms.dimension == 0u) {
            final_color *= vec4<f32>(light_intensity, light_intensity, light_intensity, 1.0); // Make it lighter
        } else {
            final_color *= vec4<f32>(dark_intensity, dark_intensity, dark_intensity, 1.0); // Make it darker
        }
    }
*/

    //return light_color;
    //return vec4<f32>(1.0, 0.0, 0.0, 1.0); // Red color// * 0.9 + dark_color * 0.1;
    return final_color;
}