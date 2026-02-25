/// 物理常数 — 这个宇宙的基本参数
///
/// 在真实宇宙里，光速、普朗克常数、引力常数
/// 是固定的，没有人知道为什么是这些值而不是别的。
/// 在这个宇宙里，你可以改变它们，然后看看会发生什么。
#[derive(Clone, Debug)]
pub struct PhysicalConstants {
    /// 时间步长 — 宇宙时钟的精度
    pub dt: f32,
    /// 空间分辨率 — 宇宙的最小尺度
    pub dx: f32,
}

impl Default for PhysicalConstants {
    fn default() -> Self {
        Self {
            dt: 1.0,
            dx: 1.0,
        }
    }
}
