use anyhow::Result;
use wgpu::*;

/// GPU 上下文 — 宇宙的计算基础设施
///
/// 现代 GPU 是一个并行计算机，有数千个核心同时运行。
/// 这正是模拟宇宙所需要的：每个格子的物理计算是独立的，
/// 可以完全并行。
pub struct GpuContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

impl GpuContext {
    pub async fn new() -> Result<Self> {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                ..Default::default()
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("找不到 GPU 适配器"))?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default(), None)
            .await?;

        log::info!("GPU: {}", adapter.get_info().name);

        Ok(Self { instance, adapter, device, queue })
    }
}
