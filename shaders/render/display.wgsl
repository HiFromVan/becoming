// display.wgsl — 把场的数值渲染成像素
//
// 这是宇宙与人眼之间的翻译层。
// 它把 0..1 的浮点数变成屏幕上的颜色。

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VertexOutput {
    // 全屏三角形
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );

    var out: VertexOutput;
    out.position = vec4<f32>(positions[vi], 0.0, 1.0);
    out.uv = uvs[vi];
    return out;
}

@group(0) @binding(0) var field_texture: texture_2d<f32>;
@group(0) @binding(1) var field_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let value = textureSample(field_texture, field_sampler, in.uv).r;

    // 蓝白色映射 — 像深海到浪尖
    let color = mix(
        vec3<f32>(0.0, 0.05, 0.2),
        vec3<f32>(0.9, 0.95, 1.0),
        value
    );

    return vec4<f32>(color, 1.0);
}
