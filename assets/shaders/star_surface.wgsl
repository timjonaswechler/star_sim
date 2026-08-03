#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::{globals, view},
}

struct StarSurfaceUniform {
    emissive: vec4<f32>,
    limb_darkening: f32,
    granulation_strength: f32,
    granulation_scale: f32,
    animation_speed: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> star: StarSurfaceUniform;

fn hash(position: vec3<f32>) -> f32 {
    return fract(sin(dot(position, vec3(127.1, 311.7, 74.7))) * 43758.5453);
}

fn value_noise(position: vec3<f32>) -> f32 {
    let cell = floor(position);
    let local = fract(position);
    let blend = local * local * (3.0 - 2.0 * local);
    let x00 = mix(hash(cell), hash(cell + vec3(1.0, 0.0, 0.0)), blend.x);
    let x10 = mix(hash(cell + vec3(0.0, 1.0, 0.0)), hash(cell + vec3(1.0, 1.0, 0.0)), blend.x);
    let x01 = mix(hash(cell + vec3(0.0, 0.0, 1.0)), hash(cell + vec3(1.0, 0.0, 1.0)), blend.x);
    let x11 = mix(hash(cell + vec3(0.0, 1.0, 1.0)), hash(cell + vec3(1.0, 1.0, 1.0)), blend.x);
    return mix(mix(x00, x10, blend.y), mix(x01, x11, blend.y), blend.z);
}

fn granulation(position: vec3<f32>) -> f32 {
    let drift = vec3(globals.time * star.animation_speed, 0.0, globals.time * star.animation_speed * 0.63);
    let p = position * star.granulation_scale + drift;
    let broad = value_noise(p);
    let fine = value_noise(p * 2.07 + vec3(19.1, 7.3, 3.7));
    return (broad * 0.7 + fine * 0.3 - 0.5) * 2.0;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.world_normal);
    let view_direction = normalize(view.world_position - input.world_position.xyz);
    let mu = max(dot(normal, view_direction), 0.0);
    let limb = max(0.02, 1.0 - star.limb_darkening * (1.0 - mu));
    let surface_noise = granulation(normal);
    let brightness = max(0.0, limb * (1.0 + star.granulation_strength * surface_noise));
    return vec4(star.emissive.rgb * brightness * view.exposure, 1.0);
}
