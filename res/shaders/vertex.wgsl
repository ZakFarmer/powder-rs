// res/shaders/vertex.wgsl

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) v_liveValue: f32,
};

@vertex
fn vs_main(
    @location(0) a_vertexPosition: vec2<f32>,
    @location(1) a_instancePosition: vec2<f32>,
    @location(2) a_instanceScale: f32,
) -> VertexOutput {
    var out: VertexOutput;

    // Apply the per-instance transformation
    var transformed_position: vec2<f32> = a_vertexPosition * a_instanceScale + a_instancePosition;

    out.clip_position = vec4<f32>(transformed_position, 0.0, 1.0);
    out.v_liveValue = 1.0;

    return out;
}
