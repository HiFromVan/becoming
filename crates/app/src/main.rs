use anyhow::Result;
use core::rule::Rule;
use universe::init::Init;

fn main() -> Result<()> {
    env_logger::init();

    log::info!("becoming — 宇宙开始运行");

    // 创建宇宙
    let mut universe = Init::default_universe(512, 512);

    log::info!(
        "规则: {}, 初始时间: {:.2}, 步长: {}",
        universe.rule.name(),
        universe.time,
        universe.dt
    );

    // 运行几步，验证基础架构
    universe.tick_n(10);

    log::info!(
        "已运行 {} 步, t = {:.2}",
        universe.step_count,
        universe.time
    );

    // TODO: 接入渲染器，让宇宙被看见
    // let gpu = pollster::block_on(renderer::GpuContext::new())?;

    println!("宇宙正在运行。");
    println!("step: {}, t: {:.2}", universe.step_count, universe.time);

    Ok(())
}
