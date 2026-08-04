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
    display_mode: u32,
    magnetic_axis: vec4<f32>,
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

fn perlin_gradient(cell: vec3<f32>) -> vec3<f32> {
    let gradient = vec3(
        hash(cell) - 0.5,
        hash(cell + vec3(19.19, 7.13, 3.71)) - 0.5,
        hash(cell + vec3(5.47, 23.17, 11.89)) - 0.5,
    );
    return normalize(gradient + vec3(0.0001));
}

fn perlin_noise(position: vec3<f32>) -> f32 {
    let cell = floor(position);
    let local = fract(position);
    let fade = local * local * local * (local * (local * 6.0 - 15.0) + 10.0);

    let n000 = dot(perlin_gradient(cell), local);
    let n100 = dot(perlin_gradient(cell + vec3(1.0, 0.0, 0.0)), local - vec3(1.0, 0.0, 0.0));
    let n010 = dot(perlin_gradient(cell + vec3(0.0, 1.0, 0.0)), local - vec3(0.0, 1.0, 0.0));
    let n110 = dot(perlin_gradient(cell + vec3(1.0, 1.0, 0.0)), local - vec3(1.0, 1.0, 0.0));
    let n001 = dot(perlin_gradient(cell + vec3(0.0, 0.0, 1.0)), local - vec3(0.0, 0.0, 1.0));
    let n101 = dot(perlin_gradient(cell + vec3(1.0, 0.0, 1.0)), local - vec3(1.0, 0.0, 1.0));
    let n011 = dot(perlin_gradient(cell + vec3(0.0, 1.0, 1.0)), local - vec3(0.0, 1.0, 1.0));
    let n111 = dot(perlin_gradient(cell + vec3(1.0, 1.0, 1.0)), local - vec3(1.0));

    let x00 = mix(n000, n100, fade.x);
    let x10 = mix(n010, n110, fade.x);
    let x01 = mix(n001, n101, fade.x);
    let x11 = mix(n011, n111, fade.x);
    return mix(mix(x00, x10, fade.y), mix(x01, x11, fade.y), fade.z) * 1.65;
}

fn layered_magnetic_noise(position: vec3<f32>) -> f32 {
    // Sample isotropic 3D noise directly on the sphere and superimpose broad,
    // medium and fine wavelengths. This has no longitude seam or polar pinch.
    let broad_f = 3.0;
    let medium_f = 9.0;
    let fine_f = 12.0;
    let broad_w = 0.2;
    let medium_w = 0.5;
    let fine_w = 1.0 - broad_w - medium_w;
    let broad = perlin_noise(position * broad_f + vec3(13.7, 3.1, 8.9));
    let medium = perlin_noise(position * medium_f + vec3(2.4, 17.2, 5.6));
    let fine = perlin_noise(position * fine_f + vec3(21.8, 4.3, 11.1));
    return clamp(broad * broad_w + medium * medium_w + fine * fine_w, -1.0, 1.0);
}

fn dipole_polarity(position: vec3<f32>) -> f32 {
    let magnetic_axis = normalize(star.magnetic_axis.xyz);
    let axis_alignment = dot(position, magnetic_axis);

    // Most of the surface is a noisy multipolar belt. Only close to the two
    // ends of the magnetic axis does the stable global polarity take over.
    let multipolar_belt = clamp(
        layered_magnetic_noise(position) * 1.75 + axis_alignment * 0.08,
        -1.0,
        1.0,
    );
    let polar_cap = smoothstep(0.72, 0.99, abs(axis_alignment));
    let stable_pole = select(-1.0, 1.0, axis_alignment >= 0.0);
    return mix(multipolar_belt, stable_pole, polar_cap);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.world_normal);
    let view_direction = normalize(view.world_position - input.world_position.xyz);
    let mu = max(dot(normal, view_direction), 0.0);
    let limb = max(0.02, 1.0 - star.limb_darkening * (1.0 - mu));
    let surface_noise = granulation(normal);
    let brightness = max(0.0, limb * (1.0 + star.granulation_strength * surface_noise));

    if star.display_mode == 1u {
        let polarity = dipole_polarity(normal);
        let negative_color = vec3(0.015, 0.055, 1.0);
        let positive_color = vec3(1.0, 0.025, 0.008);
        let polarity_mix = smoothstep(-0.12, 0.12, polarity);
        let polarity_color = mix(negative_color, positive_color, polarity_mix);
        let neutral_band = 1.0 - smoothstep(0.015, 0.11, abs(polarity));
        let diagnostic_color = mix(polarity_color, vec3(0.9), neutral_band * 0.75);
        let luminance_weights = vec3(0.2126, 0.7152, 0.0722);
        let color_luminance = max(dot(diagnostic_color, luminance_weights), 0.001);
        let normalized_diagnostic_color = diagnostic_color / color_luminance;
        let physical_luminance = dot(star.emissive.rgb, luminance_weights);
        let diagnostic_brightness = physical_luminance * limb * view.exposure;
        return vec4(normalized_diagnostic_color * diagnostic_brightness, 1.0);
    }

    return vec4(star.emissive.rgb * brightness * view.exposure, 1.0);
}
