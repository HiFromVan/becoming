use winit::window::Window;

/// 窗口状态 — 宇宙的观察窗口
///
/// winit 0.30+ 使用 ApplicationHandler 模式创建窗口，
/// 这里只持有窗口引用和尺寸信息。
pub struct WindowState {
    pub window: Window,
    pub width: u32,
    pub height: u32,
}

impl WindowState {
    pub fn from_window(window: Window) -> Self {
        let size = window.inner_size();
        Self {
            width: size.width,
            height: size.height,
            window,
        }
    }
}
