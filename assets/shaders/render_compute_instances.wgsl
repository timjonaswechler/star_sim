#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) instance_position_scale: vec4<f32>,
    @builtin(instance_index) instance_index: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

fn hash(value: u32) -> f32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ (state >> 16u);
    return f32(state) / 4294967295.0;
}

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let position = vertex.position * vertex.instance_position_scale.w
        + vertex.instance_position_scale.xyz;

    var output: VertexOutput;
    // The mesh itself has one Bevy transform. GPU instance transforms live in
    // our separate buffer, so index 0 is intentionally used for Bevy's mesh
    // transform lookup.
    output.clip_position = mesh_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(position, 1.0),
    );

    let temperature = hash(vertex.instance_index + 27u);
    let cool = vec3<f32>(0.0, 0.0, 1.0);
    let hot = vec3<f32>(1.0, 0.0, 0.0);
    output.color = mix(cool, hot, temperature) * 1.8;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
