use core::rule::Rule;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::Path;

/// 宇宙 — 规则与状态的结合
///
/// 宇宙不是规则，也不是状态，而是两者的运行。
/// 规则定义了"什么是可能的"，状态记录了"现在是什么"，
/// 而宇宙是那个让可能性不断实现的过程。
pub struct Universe<R: Rule> {
    pub rule: R,
    pub state: R::State,
    pub time: f64,
    pub step_count: u64,
    pub dt: f32,
}

impl<R: Rule> Universe<R>
where
    R::State: Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(rule: R, initial_state: R::State, dt: f32) -> Self {
        Self {
            rule,
            state: initial_state,
            time: 0.0,
            step_count: 0,
            dt,
        }
    }

    /// 让宇宙前进一步
    pub fn tick(&mut self) {
        self.state = self.rule.step(&self.state, self.dt);
        self.time += self.dt as f64;
        self.step_count += 1;
    }

    /// 让宇宙前进 n 步
    pub fn tick_n(&mut self, n: u64) {
        for _ in 0..n {
            self.tick();
        }
    }

    /// 保存宇宙当前状态到文件
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.state)?;
        std::fs::write(path, json)?;
        log::info!("宇宙状态已保存 — step {}, t={:.4}", self.step_count, self.time);
        Ok(())
    }

    /// 从文件加载宇宙状态
    pub fn load_state(&mut self, path: &Path) -> Result<()> {
        let json = std::fs::read_to_string(path)?;
        self.state = serde_json::from_str(&json)?;
        log::info!("宇宙状态已加载");
        Ok(())
    }
}
