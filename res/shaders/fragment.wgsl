// res/shaders/fragment.wgsl

struct FragmentInput {
    @location(0) v_liveValue: f32,
};

struct FragmentOutput {
    @location(0) o_color: vec4<f32>,
};

@fragment
fn fs_main(input: FragmentInput) -> FragmentOutput {
    var out: FragmentOutput;
    out.o_color = vec4<f32>(input.v_liveValue, 0.0, 0.0, 1.0);

    return out;
}