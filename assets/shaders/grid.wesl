//this shader should help to crate a grid overlay background like in https://gameuidatabase.com/gameData.php?id=617&autoload=29954


fn calculate_grid(uv: vec2<f32>, line_width: f32) -> f32 {
    let fract_uv = fract(uv);
    let cell = abs(fract_uv - 0.5) / line_width;
    let line = min(cell.x, cell.y);
    return 1.0 - smoothstep(0.0, 1.0, line);
}



@group(0) @binding(0) var<uniform> time: f32;
struct VertexOutput {
    @builtin(position) position: vec4<f32>, // Keine location(0) mehr nötig!
}

@fragment
fn fragment_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Nutzt die echten Bildschirm-Pixel (z.B. Werte von 0 bis 1920)
    // Wir teilen durch 100, damit das Gitter eine angenehme Größe hat
    let screen_uv = in.position.xy / 10.0;

    let animated_uv = screen_uv + sin(time) * 0.1;

    let thickness = .10;
    let grid_line = calculate_grid(animated_uv, thickness);
    let grid_color = vec3<f32>(0.0, 0.8, 1.0) * grid_line;
    return vec4<f32>(grid_color, 0.10);
}
