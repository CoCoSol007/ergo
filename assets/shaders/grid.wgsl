#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> color: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let grid_size = 25.0;
    let line_width = 0.5;

    let pos = in.world_position.xy;

    let d = abs(fract(pos / grid_size) - 0.5) * grid_size;

    if (d.x < line_width || d.y < line_width) {
        return color;
    }

    return vec4<f32>(0.1, 0.1, 0.1, 1.0);
}