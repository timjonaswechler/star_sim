#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::{globals, view},
}
#import "shaders/noise.wgsl"::{FnlSettings, fnl_domain_warp_3d, FNL_CELLULAR_DISTANCE_EUCLIDEAN,FNL_FRACTAL_DOMAIN_WARP_PROGRESSIVE, FNL_DOMAIN_WARP_OPEN_SIMPLEX_2_REDUCED, FNL_NOISE_OPEN_SIMPLEX_2, FNL_NOISE_CELLULAR, FNL_CELLULAR_RETURN_DISTANCE2_SUB, FNL_CELLULAR_RETURN_DISTANCE, FNL_FRACTAL_FBM, fnl_default_settings, fnl_get_noise_3d}

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

fn solar_granulation(direction: vec3<f32>, time: f32) -> f32 {
    let position = normalize(direction) * 100;

    // Verformt das Koordinatenfeld und macht die Zellen unregelmäßig.
    var warp = fnl_default_settings();
    warp.domain_warp_type = FNL_DOMAIN_WARP_OPEN_SIMPLEX_2_REDUCED;
    warp.fractal_type = FNL_FRACTAL_DOMAIN_WARP_PROGRESSIVE;
    warp.frequency = 0.8;
    warp.domain_warp_amp = 1.5;
    warp.octaves = 3;

    let animated_position = position + vec3<f32>(
        time * 0.03,
        0.0,
        time * 0.021,
    );

    let warped = fnl_domain_warp_3d(
        warp,
        animated_position.x,
        animated_position.y,
        animated_position.z,
    );

    // F2 - F1 wird an Zellgrenzen klein und im Zellinneren größer.
    var cells = fnl_default_settings();
    cells.noise_type = FNL_NOISE_CELLULAR;
    cells.cellular_distance_function = FNL_CELLULAR_DISTANCE_EUCLIDEAN;
    cells.cellular_return_type = FNL_CELLULAR_RETURN_DISTANCE2_SUB;
    cells.cellular_jitter_modifier = 0.7;
    cells.frequency = 12.0;

    let cellular_noise = fnl_get_noise_3d(
        cells,
        warped.x,
        warped.y,
        warped.z,
    );

    // F2 - F1 ist an den Zellgrenzen 0 und wächst kontinuierlich in
    // Richtung Zellinneres. Ein exponentielles Profil vermeidet die große,
    // konstante Fläche, die durch einen früh sättigenden smoothstep entsteht.
    let distance_from_lane = (cellular_noise + 1.0) * 0.5;
    let transition = 1.0 - exp(-distance_from_lane * 4);
    let cell_profile = pow(transition, 1);

    // Normalisierte Maske: 0 an dunklen Rändern, gegen 1 im hellen Inneren.
    return cell_profile;
}

fn sunspots(direction: vec3<f32>) -> f32 {
    let surface_direction = normalize(direction);

    // Liegt auf der zur Startkamera gerichteten +Z-Halbkugel. Durch Ändern
    // dieser Richtung lässt sich der Fleck frei auf der Kugel platzieren.
    let main_center = normalize(vec3<f32>(0.34, 0.16, 0.93));
    let main_radius = 0.18;

    // Niedrigfrequentes FBm verformt die ansonsten kreisförmige Kontur.
    var shape_noise = fnl_default_settings();
    shape_noise.seed = 7341;
    shape_noise.noise_type = FNL_NOISE_OPEN_SIMPLEX_2;
    shape_noise.fractal_type = FNL_FRACTAL_FBM;
    shape_noise.frequency = 0.8;
    shape_noise.octaves = 3;

    let irregularity = fnl_get_noise_3d(
        shape_noise,
        surface_direction.x * 6.0,
        surface_direction.y * 6.0,
        surface_direction.z * 6.0,
    );

    let main_distance =
        length(surface_direction - main_center) / main_radius
        + irregularity * 0.14;

    // Die Penumbra bildet den breiten, weichen Außenbereich. Die Umbra ist
    // kleiner und deutlich dunkler. Beide Masken gehen kontinuierlich über.
    let main_penumbra = 1.0 - smoothstep(0.58, 1.12, main_distance);
    let main_umbra = 1.0 - smoothstep(0.12, 0.50, main_distance);
    var main_attenuation = mix(1.0, 0.42, main_penumbra);
    main_attenuation = mix(main_attenuation, 0.045, main_umbra);

    // Kleiner Begleitfleck, damit die Formation weniger isoliert wirkt.
    let companion_center = normalize(vec3<f32>(0.50, 0.10, 0.86));
    let companion_distance =
        length(surface_direction - companion_center) / 0.065
        - irregularity * 0.10;
    let companion_penumbra = 1.0 - smoothstep(0.55, 1.10, companion_distance);
    let companion_umbra = 1.0 - smoothstep(0.10, 0.46, companion_distance);
    var companion_attenuation = mix(1.0, 0.48, companion_penumbra);
    companion_attenuation = mix(companion_attenuation, 0.07, companion_umbra);

    // 1.0 lässt die Photosphäre unverändert; kleinere Werte dunkeln sie ab.
    return min(main_attenuation, companion_attenuation);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let normal = normalize(input.world_normal);
    let view_direction = normalize(view.world_position - input.world_position.xyz);
    let mu = max(dot(normal, view_direction), 0.0);
    let limb = max(0.02, 1.0 - star.limb_darkening * (1.0 - mu));
    let granulation = solar_granulation(normal, globals.time);
    let granulation_brightness = mix(0.45, 1.55, granulation);
    let brightness = limb * granulation_brightness;

    let dark_color = star.emissive.rgb * 0.35;
    let bright_color = star.emissive.rgb * 1.45;
    let surface_color = mix(dark_color, bright_color, granulation);
    let spot_attenuation = sunspots(normal);
    let color = surface_color * spot_attenuation;

    return vec4(
        color * brightness * view.exposure,
        1.0,
    );
}
