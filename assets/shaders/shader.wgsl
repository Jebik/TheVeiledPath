struct ShaderData {
    player_position: vec2<f32>,
    player_direction: vec2<f32>,
    goal_position: vec2<f32>,
    size_info: vec2<f32>,
    color: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> data: ShaderData;

@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var txt_sampler: sampler;


@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let world_color = textureSample(texture, txt_sampler, uv);

    var final_color = data.color;
    // Transform uv coordinates to world space
    var uv_world = uv * data.size_info;    
    // Correct for the 0.5 offset
    uv_world -= vec2<f32>(0.5, 0.5);

    let player_distance = distance(data.player_position, uv_world);
    let goal_distance = distance(data.goal_position, uv_world);
    // Draw a circle with a radius of 2.0 around player and goal
    if (player_distance <= 1.5 || goal_distance <= 1.5) {
        final_color = world_color;
    }

    // Create a directional cone for the player
    let direction_to_pixel = normalize(uv_world - data.player_position);
    let cosine_angle = dot(data.player_direction, direction_to_pixel);

    // Calculate a blend factor based on the distance to the player, range is [0, 1]
    let blend_factor = clamp(1.0 - ((player_distance - 1.5) / (8.0 - 1.5)), 0.0, 1.0);

    // This will create a cone with an angle of about 60 degrees (cos(45 degrees) = 0.707)
    // You can adjust this value to make the cone wider or narrower
    if (cosine_angle > 0.707 && player_distance <= 8.0) {
        // Blend the final color with the light color based on the blend factor
        final_color = mix(final_color, world_color, blend_factor);
    }

    return final_color;
}