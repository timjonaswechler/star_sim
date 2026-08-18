// this shader should only greate fine streamer like in polar lights https://gameuidatabase.com/gameData.php?id=175&autoload=6476
// goal also is to create over exposure streams like in https://gameuidatabase.com/gameData.php?id=1880&autoload=75464

#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::{globals, view},
    view_transformations::{frag_coord_to_ndc, position_ndc_to_world},
}

const SAMPLE_COUNT: u32 = 40u;

// -----------------------------------------------------------------------------
// Streamer tuning
// -----------------------------------------------------------------------------
// Broad + medium layers use simplex noise for flowing, non-blocky shapes.
// The fine layer deliberately keeps value noise for small, sharp structure.
// Lower angular frequency = fewer, wider structures around the star.
const BROAD_ANGULAR_FREQUENCY: f32 = 0.70;
const MEDIUM_ANGULAR_FREQUENCY: f32 = 2.3;
const FINE_ANGULAR_FREQUENCY: f32 = 5.1;
const BROAD_NOISE_WEIGHT: f32 = 0.48;
const MEDIUM_NOISE_WEIGHT: f32 = 0.34;
const FINE_NOISE_WEIGHT: f32 = 0.18;

// Radial reach is a fraction of the corona thickness: 0.0 is the stellar
// surface and 1.0 is the volume's outer radius. Fade width controls how softly
// each layer disappears before its reach; it must be smaller than the reach.
const BROAD_RADIAL_REACH: f32 = 0.95;
const BROAD_RADIAL_FADE_WIDTH: f32 = 0.18;
const MEDIUM_RADIAL_REACH: f32 = 0.68;
const MEDIUM_RADIAL_FADE_WIDTH: f32 = 0.18;
const FINE_RADIAL_REACH: f32 = 0.42;
const FINE_RADIAL_FADE_WIDTH: f32 = 0.14;

// A smaller interval creates fewer, sharper radial streamer channels.
const STREAMER_RIDGE_START: f32 = 0.50;
const STREAMER_RIDGE_END: f32 = 0.72;
const STREAMER_RIDGE_POWER: f32 = 5.4;
const STREAMER_STRENGTH: f32 = 5.35;
const BACKGROUND_DENSITY: f32 = 0.22;

// Makes streamers more prominent away from the stellar surface.
const HEIGHT_EMPHASIS_START: f32 = 0.04;
const HEIGHT_EMPHASIS_END: f32 = 0.58;
const HEIGHT_EMPHASIS_NEAR: f32 = 0.9;
const HEIGHT_EMPHASIS_FAR: f32 = 2.0;

// Converts temporal drift into movement through the noise domains.
const DRIFT_SPATIAL_SCALE: f32 = 10.0;
const OUTER_FADE_START_FRACTION: f32 = 0.72;

struct CoronaVolumeUniform {
    emissive: vec4<f32>,
    appearance: vec4<f32>,
    geometry: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> corona: CoronaVolumeUniform;

fn value_noise_hash(position: vec3<f32>) -> f32 {
    return fract(sin(dot(position, vec3(127.1, 311.7, 74.7))) * 43758.5453);
}

// Fine-detail noise. Returns a value in [0, 1].
fn value_noise_3d(position: vec3<f32>) -> f32 {
    let cell = floor(position);
    let local = fract(position);
    let blend = local * local * (3.0 - 2.0 * local);
    let x00 = mix(value_noise_hash(cell), value_noise_hash(cell + vec3(1.0, 0.0, 0.0)), blend.x);
    let x10 = mix(value_noise_hash(cell + vec3(0.0, 1.0, 0.0)), value_noise_hash(cell + vec3(1.0, 1.0, 0.0)), blend.x);
    let x01 = mix(value_noise_hash(cell + vec3(0.0, 0.0, 1.0)), value_noise_hash(cell + vec3(1.0, 0.0, 1.0)), blend.x);
    let x11 = mix(value_noise_hash(cell + vec3(0.0, 1.0, 1.0)), value_noise_hash(cell + vec3(1.0, 1.0, 1.0)), blend.x);
    return mix(mix(x00, x10, blend.y), mix(x01, x11, blend.y), blend.z);
}

// 3D simplex noise adapted from the MIT-licensed wgsl-fns implementation:
// https://dekoolecentrale.nl/wgsl-fns/simplexNoise3D
// Returns approximately [-1, 1].
fn simplex_permute(value: vec4<f32>) -> vec4<f32> {
    return ((value * 34.0 + 1.0) * value) % vec4(289.0);
}

fn simplex_inverse_sqrt(value: vec4<f32>) -> vec4<f32> {
    return 1.79284291400159 - 0.85373472095314 * value;
}

fn simplex_noise_3d(position: vec3<f32>) -> f32 {
    let skew = vec2(1.0 / 6.0, 1.0 / 3.0);
    let offsets = vec4(0.0, 0.5, 1.0, 2.0);
    var cell = floor(position + dot(position, skew.yyy));
    let corner0 = position - cell + dot(cell, skew.xxx);
    let rank = step(corner0.yzx, corner0.xyz);
    let inverse_rank = 1.0 - rank;
    let corner_rank1 = min(rank.xyz, inverse_rank.zxy);
    let corner_rank2 = max(rank.xyz, inverse_rank.zxy);
    let corner1 = corner0 - corner_rank1 + skew.xxx;
    let corner2 = corner0 - corner_rank2 + 2.0 * skew.xxx;
    let corner3 = corner0 - 1.0 + 3.0 * skew.xxx;

    cell = cell % vec3(289.0);
    let permutation = simplex_permute(simplex_permute(simplex_permute(
        cell.z + vec4(0.0, corner_rank1.z, corner_rank2.z, 1.0))
        + cell.y + vec4(0.0, corner_rank1.y, corner_rank2.y, 1.0))
        + cell.x + vec4(0.0, corner_rank1.x, corner_rank2.x, 1.0));

    let gradient_scale = (1.0 / 7.0) * offsets.wyz - offsets.xzx;
    let gradient_index = permutation - 49.0 * floor(permutation * gradient_scale.z * gradient_scale.z);
    let gradient_x_index = floor(gradient_index * gradient_scale.z);
    let gradient_y_index = floor(gradient_index - 7.0 * gradient_x_index);
    let gradient_x = gradient_x_index * gradient_scale.x + gradient_scale.yyyy;
    let gradient_y = gradient_y_index * gradient_scale.x + gradient_scale.yyyy;
    let gradient_height = 1.0 - abs(gradient_x) - abs(gradient_y);
    let gradient_pair0 = vec4(gradient_x.xy, gradient_y.xy);
    let gradient_pair1 = vec4(gradient_x.zw, gradient_y.zw);
    let gradient_sign0 = floor(gradient_pair0) * 2.0 + 1.0;
    let gradient_sign1 = floor(gradient_pair1) * 2.0 + 1.0;
    let gradient_mask = -step(gradient_height, vec4(0.0));
    let gradient0 = gradient_pair0.xzyw + gradient_sign0.xzyw * gradient_mask.xxyy;
    let gradient1 = gradient_pair1.xzyw + gradient_sign1.xzyw * gradient_mask.zzww;

    var direction0 = vec3(gradient0.xy, gradient_height.x);
    var direction1 = vec3(gradient0.zw, gradient_height.y);
    var direction2 = vec3(gradient1.xy, gradient_height.z);
    var direction3 = vec3(gradient1.zw, gradient_height.w);
    let normalization = simplex_inverse_sqrt(vec4(
        dot(direction0, direction0), dot(direction1, direction1),
        dot(direction2, direction2), dot(direction3, direction3),
    ));
    direction0 *= normalization.x;
    direction1 *= normalization.y;
    direction2 *= normalization.z;
    direction3 *= normalization.w;

    var influence = max(0.6 - vec4(
        dot(corner0, corner0), dot(corner1, corner1),
        dot(corner2, corner2), dot(corner3, corner3),
    ), vec4(0.0));
    influence *= influence;
    return 42.0 * dot(influence * influence, vec4(
        dot(direction0, corner0), dot(direction1, corner1),
        dot(direction2, corner2), dot(direction3, corner3),
    ));
}

// This is the seam for changing the large streamer shapes independently from
// fine detail. It normalizes simplex noise to the same [0, 1] range as value noise.
fn broad_streamer_noise(position: vec3<f32>) -> f32 {
    return clamp(simplex_noise_3d(position) * 0.5 + 0.5, 0.0, 1.0);
}

fn fine_streamer_noise(position: vec3<f32>) -> f32 {
    return value_noise_3d(position);
}

// Returns 1 near the surface and fades to 0 at radial_reach.
fn radial_reach_mask(
    normalized_height: f32,
    radial_reach: f32,
    fade_width: f32,
) -> f32 {
    let fade_start = max(radial_reach - fade_width, 0.0);
    return 1.0 - smoothstep(fade_start, radial_reach, normalized_height);
}

fn streamer_density(
    sample_position: vec3<f32>,
    stellar_radius: f32,
    volume_outer_radius: f32,
    angular_noise_scale: f32,
    noise_drift: vec3<f32>,
) -> f32 {
    let radial_direction = normalize(sample_position);
    let height_above_surface = max(length(sample_position) - stellar_radius, 0.0);
    let corona_thickness = max(volume_outer_radius - stellar_radius, 0.0001);
    let normalized_height = clamp(height_above_surface / corona_thickness, 0.0, 1.0);

    // Sampling only the direction keeps a noise region coherent along a radial ray.
    let angular_position = radial_direction * angular_noise_scale;
    let broad_shape = broad_streamer_noise(
        angular_position * BROAD_ANGULAR_FREQUENCY + noise_drift,
    );
    let medium_shape = broad_streamer_noise(
        angular_position * MEDIUM_ANGULAR_FREQUENCY
        + vec3(17.3, 4.7, 9.1)
        + noise_drift * 0.45,
    );
    let fine_detail = fine_streamer_noise(
        angular_position * FINE_ANGULAR_FREQUENCY
        + vec3(3.8, 23.1, 12.7)
        - noise_drift * 0.28,
    );

    let broad_reach = radial_reach_mask(
        normalized_height,
        BROAD_RADIAL_REACH,
        BROAD_RADIAL_FADE_WIDTH,
    );
    let medium_reach = radial_reach_mask(
        normalized_height,
        MEDIUM_RADIAL_REACH,
        MEDIUM_RADIAL_FADE_WIDTH,
    );
    let fine_reach = radial_reach_mask(
        normalized_height,
        FINE_RADIAL_REACH,
        FINE_RADIAL_FADE_WIDTH,
    );

    // A layer fades toward neutral noise (0.5), rather than zero. This keeps
    // the ridge threshold stable while progressively removing that scale's
    // structure; broad rays can therefore continue after fine detail ends.
    let broad_channel = mix(0.5, broad_shape, broad_reach);
    let medium_channel = mix(0.5, medium_shape, medium_reach);
    let fine_channel = mix(0.5, fine_detail, fine_reach);
    let combined_channels = broad_channel * BROAD_NOISE_WEIGHT
        + medium_channel * MEDIUM_NOISE_WEIGHT
        + fine_channel * FINE_NOISE_WEIGHT;
    let sharp_channels = pow(
        smoothstep(STREAMER_RIDGE_START, STREAMER_RIDGE_END, combined_channels),
        STREAMER_RIDGE_POWER,
    );
    let height_emphasis = mix(
        HEIGHT_EMPHASIS_NEAR,
        HEIGHT_EMPHASIS_FAR,
        smoothstep(HEIGHT_EMPHASIS_START, HEIGHT_EMPHASIS_END, height_above_surface),
    );
    let radial_streamers = sharp_channels * height_emphasis;

    let background_variation = mix(
        0.72,
        1.18,
        fine_streamer_noise(radial_direction * 1.2 + noise_drift * 0.2),
    );
    return BACKGROUND_DENSITY * background_variation
        + STREAMER_STRENGTH * radial_streamers;
}

fn sphere_intersections(origin: vec3<f32>, direction: vec3<f32>, radius: f32) -> vec2<f32> {
    let half_b = dot(origin, direction);
    let c = dot(origin, origin) - radius * radius;
    let discriminant = half_b * half_b - c;
    if discriminant <= 0.0 {
        return vec2(-1.0);
    }
    let root = sqrt(discriminant);
    return vec2(-half_b - root, -half_b + root);
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front_facing: bool) -> @location(0) vec4<f32> {
    // Reconstruct the ray from the fragment coordinate, not interpolated mesh
    // positions. This makes every triangle evaluate the same ray per pixel.
    // Uniform packing (kept as vec4 fields to match Rust's 16-byte layout):
    // appearance = intensity, density falloff, angular noise scale, animation speed
    // geometry   = stellar radius, volume outer radius, reserved, reserved
    let volume_intensity = corona.appearance.x;
    let density_falloff = corona.appearance.y;
    let angular_noise_scale = corona.appearance.z;
    let animation_speed = corona.appearance.w;
    let stellar_radius = corona.geometry.x;
    let volume_outer_radius = corona.geometry.y;

    let camera_position = view.world_position;
    let ndc = frag_coord_to_ndc(input.position);
    let far_world = position_ndc_to_world(vec3(ndc.xy, 1.0));
    let ray_direction = normalize(far_world - camera_position);
    let outer_hits = sphere_intersections(camera_position, ray_direction, volume_outer_radius);
    let camera_distance = length(camera_position);
    let camera_inside_volume = camera_distance < volume_outer_radius;

    // Switch to the back faces slightly before the mathematical boundary. The
    // margin is only a raster carrier guard: exact intersections below still
    // determine whether and where density exists.
    let use_inner_faces = camera_distance < volume_outer_radius + 0.12;
    if (!use_inner_faces && !front_facing) || (use_inner_faces && front_facing) {
        discard;
    }

    var ray_start = select(max(outer_hits.x, 0.0), 0.0, camera_inside_volume);
    var ray_end = outer_hits.y;
    let surface_hits = sphere_intersections(camera_position, ray_direction, stellar_radius);
    if surface_hits.x > ray_start {
        ray_end = min(ray_end, surface_hits.x);
    } else if camera_inside_volume && surface_hits.y > 0.0 && camera_distance < stellar_radius {
        // A camera inside the stellar sphere is outside the modeled corona;
        // resume integration only after exiting the opaque stellar interior.
        ray_start = max(ray_start, surface_hits.y);
    }

    let ray_length = ray_end - ray_start;
    if ray_length <= 0.0 || volume_intensity <= 0.0 {
        discard;
    }

    let step_length = ray_length / f32(SAMPLE_COUNT);
    let animation_phase = globals.time * animation_speed;
    let noise_drift = vec3(
        animation_phase,
        -animation_phase * 0.43,
        animation_phase * 0.71,
    ) * DRIFT_SPATIAL_SCALE;
    var optical_emission = 0.0;

    for (var index = 0u; index < SAMPLE_COUNT; index += 1u) {
        let distance = ray_start + (f32(index) + 0.5) * step_length;
        let position = camera_position + ray_direction * distance;
        let radius = length(position);
        let height_above_surface = max(radius - stellar_radius, 0.0);
        let radial_density = exp(-height_above_surface * density_falloff);
        // The raymarch volume has finite geometry, while exponential density
        // never reaches zero. Fade the final quarter of the corona smoothly to
        // zero so tangent rays cannot reveal the bounding sphere.
        let corona_thickness = volume_outer_radius - stellar_radius;
        let outer_fade_start = stellar_radius
            + corona_thickness * OUTER_FADE_START_FRACTION;
        let outer_fade = 1.0 - smoothstep(outer_fade_start, volume_outer_radius, radius);
        let structure = streamer_density(
            position,
            stellar_radius,
            volume_outer_radius,
            angular_noise_scale,
            noise_drift,
        );
        optical_emission += radial_density * outer_fade * structure * step_length;
    }

    let intensity = optical_emission * volume_intensity;
    return vec4(corona.emissive.rgb * view.exposure * intensity, intensity);
}
