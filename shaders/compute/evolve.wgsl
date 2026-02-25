// evolve.wgsl — 反应扩散的 GPU 计算着色器
//
// 这个着色器在 GPU 上并行计算每个格子的下一个状态。
// 数千个核心同时运行，每个核心负责一个格子。
// 这就是为什么 GPU 能模拟百万格子的宇宙。

struct Params {
    width: u32,
    height: u32,
    du: f32,
    dv: f32,
    f: f32,
    k: f32,
    dt: f32,
    _pad: f32,
}

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> u_in: array<f32>;
@group(0) @binding(2) var<storage, read> v_in: array<f32>;
@group(0) @binding(3) var<storage, read_write> u_out: array<f32>;
@group(0) @binding(4) var<storage, read_write> v_out: array<f32>;

fn idx(x: u32, y: u32) -> u32 {
    return y * params.width + x;
}

fn wrap_x(x: i32) -> u32 {
    return u32((x + i32(params.width)) % i32(params.width));
}

fn wrap_y(y: i32) -> u32 {
    return u32((y + i32(params.height)) % i32(params.height));
}

fn laplacian_u(x: u32, y: u32) -> f32 {
    let xi = i32(x);
    let yi = i32(y);
    return u_in[idx(wrap_x(xi-1), y)]
         + u_in[idx(wrap_x(xi+1), y)]
         + u_in[idx(x, wrap_y(yi-1))]
         + u_in[idx(x, wrap_y(yi+1))]
         - 4.0 * u_in[idx(x, y)];
}

fn laplacian_v(x: u32, y: u32) -> f32 {
    let xi = i32(x);
    let yi = i32(y);
    return v_in[idx(wrap_x(xi-1), y)]
         + v_in[idx(wrap_x(xi+1), y)]
         + v_in[idx(x, wrap_y(yi-1))]
         + v_in[idx(x, wrap_y(yi+1))]
         - 4.0 * v_in[idx(x, y)];
}

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let x = gid.x;
    let y = gid.y;

    if x >= params.width || y >= params.height {
        return;
    }

    let u = u_in[idx(x, y)];
    let v = v_in[idx(x, y)];

    let lap_u = laplacian_u(x, y);
    let lap_v = laplacian_v(x, y);

    let reaction = u * v * v;

    let du = params.du * lap_u - reaction + params.f * (1.0 - u);
    let dv = params.dv * lap_v + reaction - (params.f + params.k) * v;

    u_out[idx(x, y)] = clamp(u + du * params.dt, 0.0, 1.0);
    v_out[idx(x, y)] = clamp(v + dv * params.dt, 0.0, 1.0);
}
