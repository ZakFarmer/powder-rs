struct FragmentOutput {
    @location(0) o_color: vec4<f32>,
};

@fragment
fn fs_main() -> FragmentOutput {
    var out: FragmentOutput;
    out.o_color = vec4<f32>(1.0, 0.0, 0.0, 1.0);  // Red color
    return out;
}