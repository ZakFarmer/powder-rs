struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) a_position: vec2<f32>
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(a_position, 0.0, 1.0);
    return out;
}