//! renderer — 让宇宙被看见
//!
//! 物理规则在黑暗中运行，渲染器是那扇窗。
//! 它把场的数值映射成颜色，把时间的流动映射成画面，
//! 让不可见的数学变成可以被感受的视觉现象。

pub mod gpu;
pub mod window;
pub mod colormap;

pub use gpu::GpuContext;
pub use window::WindowState;
