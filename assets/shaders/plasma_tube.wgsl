#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::{globals, view},
}

struct PlasmaTubeUniform {
    color: vec4<f32>,
    dynamics: vec4<f32>,
    detail: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> plasma: PlasmaTubeUniform;

fn hash(position: vec2<f32>) -> f32 {
    return fract(sin(dot(position, vec2(127.1, 311.7))) * 43758.5453);
}

fn value_noise(position: vec2<f32>) -> f32 {
    let cell = floor(position);
    let local = fract(position);
    let blend = local * local * (3.0 - 2.0 * local);
    return mix(
        mix(hash(cell), hash(cell + vec2(1.0, 0.0)), blend.x),
        mix(hash(cell + vec2(0.0, 1.0)), hash(cell + vec2(1.0, 1.0)), blend.x),
        blend.y,
    );
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let brightness = plasma.dynamics.x;
    let fill = plasma.dynamics.y;
    let speed = plasma.dynamics.z;
    let phase_offset = plasma.dynamics.w;
    let detail_scale = plasma.detail.x;
    let turbulence = plasma.detail.y;

    // Energy reaches the loop from both footpoints during the evaporation phase.
    let distance_from_footpoint = 2.0 * min(input.uv.x, 1.0 - input.uv.x);
    if distance_from_footpoint > fill {
        discard;
    }

    let flow_position = input.uv.x * detail_scale - globals.time * speed + phase_offset;
    let broad = value_noise(vec2(flow_position, input.uv.y * 3.0 + phase_offset));
    let fine = value_noise(vec2(flow_position * 2.37 + 9.1, input.uv.y * 7.0));
    let moving_packet = pow(0.5 + 0.5 * sin(flow_position * 1.55 + broad * 4.0), 2.0);
    let clumped_plasma = 0.14 + broad * 0.42 + fine * 0.12 + moving_packet * 0.48;
    let density = mix(1.0, clumped_plasma, turbulence *2);

    // A bright edge and softer core make the mesh read like a luminous volume.
    let normal = normalize(input.world_normal);
    let view_direction = normalize(view.world_position - input.world_position.xyz);
    let rim = pow(1.0 - abs(dot(normal, view_direction)), 1.4);
    let volume_profile = 0.58 + rim * 0.85;
    let leading_edge = 1.0 + 0.75 * smoothstep(fill - 0.1, fill, distance_from_footpoint);
    let emission = plasma.color.rgb * brightness * density * volume_profile * leading_edge;
    let opacity = clamp(0.16 + density * 0.2 + rim * 0.17, 0.0, 0.62);
    return vec4(emission, opacity);
}
