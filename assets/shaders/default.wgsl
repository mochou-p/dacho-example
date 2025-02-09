// dacho-example/assets/shaders/default.wgsl

struct UniformBufferObject {
    view:       mat4x4<f32>,
    proj:       mat4x4<f32>,
    camera_pos: vec4<f32>,
    time:       f32
}

@group(0) @binding(0) var<uniform> ubo: UniformBufferObject;

struct VertexInput {
    @location(0) pos:    vec3<f32>,
    @location(1) normal: vec3<f32>,

    @location(2) model_row1: vec4<f32>,
    @location(3) model_row2: vec4<f32>,
    @location(4) model_row3: vec4<f32>,
    @location(5) model_row4: vec4<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,

    @location(0) normal: vec3<f32>
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let pos = vec4<f32>(in.pos, 1.0);

    var out: VertexOutput;

    let model = mat4x4<f32>(
        in.model_row1,
        in.model_row2,
        in.model_row3,
        in.model_row4
    );

    out.position = ubo.proj * (ubo.view * (model * pos));
    out.normal   = in.normal;

    return out;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>
}

@fragment
fn fragment(in: VertexOutput) -> FragmentOutput {
    var out: FragmentOutput;

    out.color = vec4<f32>(
        abs(in.normal),
        1.0
    );

    return out;
}

