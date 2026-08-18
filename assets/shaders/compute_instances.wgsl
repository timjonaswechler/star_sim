// Generates one position and scale for every mesh instance.
// The output layout is exactly one vec4<f32>, so the same GPU buffer can be
// consumed directly as an instanced vertex buffer by the render pipeline.

struct Parameters {
    instance_count: u32,
    time: f32,
    radius: f32,
    padding: f32,
}

struct Instance {
    position: vec3<f32>,
    scale: f32,
}

@group(0) @binding(0)
var<uniform> parameters: Parameters;

@group(0) @binding(1)
var<storage, read_write> instances: array<Instance>;

fn hash(value: u32) -> f32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ (state >> 16u);
    state = state * 2654435769u;
    state = state ^ (state >> 16u);
    return f32(state) / 4294967295.0;
}

@compute
@workgroup_size(64)
fn place_instances(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if index >= parameters.instance_count {
        return;
    }

    let random_radius = hash(index * 3u);
    let random_angle = hash(index * 3u + 1u);
    let random_height = hash(index * 3u + 2u);

    // sqrt() distributes points across the area of the disk instead of
    // concentrating them at its center. The radius-dependent angle creates
    // loose spiral arms.
    let radius_fraction = sqrt(random_radius);
    let distance = radius_fraction * parameters.radius;
    let angle = random_angle * 6.28318530718
        + radius_fraction * 7.0
        + parameters.time * (0.08 - radius_fraction * 0.04);
    let height = (random_height - 0.5) * (1.2 - radius_fraction * 0.8);

    instances[index].position = vec3<f32>(
        cos(angle) * distance,
        height,
        sin(angle) * distance,
    );
    instances[index].scale = 0.35 + hash(index + 9187u) * 0.65;
}
