//! 引力 — 最简单的吸引
//!
//! 一个粒子，一个质量中心，一条定律：
//! F = G * m / r²，方向指向中心。
//!
//! 这是宇宙第一次知道"距离"意味着什么。

use core::rule::Rule;
use serde::{Deserialize, Serialize};

/// 粒子的状态：位置 + 上一步位置
///
/// Verlet 积分不需要速度作为基本量——
/// 速度从两步位置之差中自然浮现。
/// 但我们仍然存它，方便读取。
#[derive(Clone, Serialize, Deserialize)]
pub struct ParticleState {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    /// 上一步的位置，Verlet 积分所需
    pub prev_x: f32,
    pub prev_y: f32,
}

/// 引力规则
///
/// 质量中心固定在原点，粒子绕它运动。
/// G 是引力常数，mass 是中心质量。
pub struct Gravity {
    pub g: f32,
    pub mass: f32,
}

impl Gravity {
    pub fn new(g: f32, mass: f32) -> Self {
        Self { g, mass }
    }
}

impl Rule for Gravity {
    type State = ParticleState;

    fn step(&self, state: &Self::State, dt: f32) -> Self::State {
        let r2 = state.x * state.x + state.y * state.y;
        let r = r2.sqrt().max(1e-4);

        let a = self.g * self.mass / r2;
        let ax = -a * state.x / r;
        let ay = -a * state.y / r;

        // Verlet 积分：x(t+dt) = 2*x(t) - x(t-dt) + a*dt²
        let next_x = 2.0 * state.x - state.prev_x + ax * dt * dt;
        let next_y = 2.0 * state.y - state.prev_y + ay * dt * dt;

        // 速度从位置差估算（用于输出，不参与积分）
        let vx = (next_x - state.prev_x) / (2.0 * dt);
        let vy = (next_y - state.prev_y) / (2.0 * dt);

        ParticleState {
            x: next_x,
            y: next_y,
            vx,
            vy,
            prev_x: state.x,
            prev_y: state.y,
        }
    }

    fn name(&self) -> &str {
        "gravity"
    }
}
