#import bevy_pbr::{
    forward_io::VertexOutput,
    mesh_view_bindings::{globals, view},
    view_transformations::{frag_coord_to_ndc, position_ndc_to_world},
}

const SAMPLE_COUNT: u32 = 40u;

struct CoronaVolumeUniform {
    emissive: vec4<f32>,
    appearance: vec4<f32>,
    geometry: vec4<f32>,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0)
var<uniform> corona: CoronaVolumeUniform;

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

fn streamer_density(position: vec3<f32>, drift: vec3<f32>) -> f32 {
    let direction = normalize(position);
    let height = max(length(position) - corona.geometry.x, 0.0);

    // Angular sampling keeps each noise region coherent along a radial ray.
    let angular = direction * corona.appearance.z;
    let broad = value_noise(angular + drift);
    let medium = value_noise(angular * 2.3 + vec3(17.3, 4.7, 9.1) + drift * 0.45);
    let fine = value_noise(angular * 5.1 + vec3(3.8, 23.1, 12.7) - drift * 0.28);

    // Keep the angular mask nearly constant along a radial ray and sharpen its
    // brightest ridges. Unlike the rejected per-region reach mask, this does
    // not turn broad angular noise cells into hard radial wedges.
    let channel_signal = broad * 0.48 + medium * 0.34 + fine * 0.18;
    let channel_ridge = pow(smoothstep(0.50, 0.72, channel_signal), 2.4);
    let height_emphasis = mix(0.9, 2.0, smoothstep(0.04, 0.58, height));
    let radial_filaments = channel_ridge * height_emphasis;

    let diffuse_variation = mix(0.72, 1.18, value_noise(direction * 1.2 + drift * 0.2));
    return 0.22 * diffuse_variation + radial_filaments * 1.35;
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
    let camera_position = view.world_position;
    let ndc = frag_coord_to_ndc(input.position);
    let far_world = position_ndc_to_world(vec3(ndc.xy, 1.0));
    let ray_direction = normalize(far_world - camera_position);
    let outer_hits = sphere_intersections(camera_position, ray_direction, corona.geometry.y);
    let camera_distance = length(camera_position);
    let camera_inside_volume = camera_distance < corona.geometry.y;

    // Switch to the back faces slightly before the mathematical boundary. The
    // margin is only a raster carrier guard: exact intersections below still
    // determine whether and where density exists.
    let use_inner_faces = camera_distance < corona.geometry.y + 0.12;
    if (!use_inner_faces && !front_facing) || (use_inner_faces && front_facing) {
        discard;
    }

    var ray_start = select(max(outer_hits.x, 0.0), 0.0, camera_inside_volume);
    var ray_end = outer_hits.y;
    let surface_hits = sphere_intersections(camera_position, ray_direction, corona.geometry.x);
    if surface_hits.x > ray_start {
        ray_end = min(ray_end, surface_hits.x);
    } else if camera_inside_volume && surface_hits.y > 0.0 && length(camera_position) < corona.geometry.x {
        // A camera inside the stellar sphere is outside the modeled corona;
        // resume integration only after exiting the opaque stellar interior.
        ray_start = max(ray_start, surface_hits.y);
    }

    let ray_length = ray_end - ray_start;
    if ray_length <= 0.0 || corona.appearance.x <= 0.0 {
        discard;
    }

    let step_length = ray_length / f32(SAMPLE_COUNT);
    let drift = vec3(
        globals.time * corona.appearance.w,
        -globals.time * corona.appearance.w * 0.43,
        globals.time * corona.appearance.w * 0.71,
    );
    var optical_emission = 0.0;

    for (var index = 0u; index < SAMPLE_COUNT; index += 1u) {
        let distance = ray_start + (f32(index) + 0.5) * step_length;
        let position = camera_position + ray_direction * distance;
        let radius = length(position);
        let height = max(radius - corona.geometry.x, 0.0);
        let radial_density = exp(-height * corona.appearance.y);
        // The raymarch volume has finite geometry, while exponential density
        // never reaches zero. Fade the final quarter of the corona smoothly to
        // zero so tangent rays cannot reveal the bounding sphere.
        let corona_thickness = corona.geometry.y - corona.geometry.x;
        let outer_fade_start = corona.geometry.x + corona_thickness * 0.72;
        let outer_fade = 1.0 - smoothstep(outer_fade_start, corona.geometry.y, radius);
        let structure = streamer_density(position, drift);
        optical_emission += radial_density * outer_fade * structure * step_length;
    }

    let intensity = optical_emission * corona.appearance.x;
    return vec4(corona.emissive.rgb * view.exposure * intensity, intensity);
}
