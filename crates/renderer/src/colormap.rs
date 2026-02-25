/// 颜色映射 — 把数值变成颜色
///
/// 一个 0 到 1 之间的数字本身没有颜色，
/// 颜色映射是一种选择：你决定用什么颜色来表达这个宇宙。
/// 不同的颜色映射会让同一个宇宙看起来完全不同。

/// 将 0..1 的值映射为 RGBA 颜色
pub trait Colormap: Send + Sync {
    fn map(&self, value: f32) -> [u8; 4];
}

/// 灰度
pub struct Grayscale;
impl Colormap for Grayscale {
    fn map(&self, value: f32) -> [u8; 4] {
        let v = (value.clamp(0.0, 1.0) * 255.0) as u8;
        [v, v, v, 255]
    }
}

/// 蓝白渐变 — 像深海到浪尖
pub struct OceanBlue;
impl Colormap for OceanBlue {
    fn map(&self, value: f32) -> [u8; 4] {
        let t = value.clamp(0.0, 1.0);
        let r = (t * 30.0) as u8;
        let g = (t * 100.0) as u8;
        let b = (50.0 + t * 205.0) as u8;
        [r, g, b, 255]
    }
}

/// 火焰 — 从黑到红到黄到白
pub struct Inferno;
impl Colormap for Inferno {
    fn map(&self, value: f32) -> [u8; 4] {
        let t = value.clamp(0.0, 1.0);
        let r = (t * 255.0) as u8;
        let g = (t * t * 200.0) as u8;
        let b = if t < 0.5 { (t * 100.0) as u8 } else { ((t - 0.5) * 2.0 * 255.0) as u8 };
        [r, g, b, 255]
    }
}
