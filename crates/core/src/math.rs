/// 数学工具
///
/// 宇宙用数学说话。这里是它的词汇。

pub use glam::Vec2;
pub use glam::Vec3;
pub use glam::Vec4;

/// 拉普拉斯算子 — 场的"曲率"，扩散的数学语言
///
/// 在连续空间里，∇²f 描述一个点与周围邻居的差异。
/// 热量从高处流向低处，化学物质从密处扩散到疏处，
/// 都是这个算子在驱动。
pub fn laplacian(data: &[f32], x: u32, y: u32, width: u32, height: u32) -> f32 {
    let idx = |x: u32, y: u32| (y * width + x) as usize;

    let left  = if x > 0          { data[idx(x - 1, y)] } else { data[idx(width - 1, y)] };
    let right = if x < width - 1  { data[idx(x + 1, y)] } else { data[idx(0, y)] };
    let up    = if y > 0          { data[idx(x, y - 1)] } else { data[idx(x, height - 1)] };
    let down  = if y < height - 1 { data[idx(x, y + 1)] } else { data[idx(x, 0)] };
    let center = data[idx(x, y)];

    left + right + up + down - 4.0 * center
}
