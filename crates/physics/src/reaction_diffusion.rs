//! 反应扩散系统
//!
//! 两种化学物质 U 和 V，在空间中扩散，同时相互反应。
//! 规则极其简单，但从中长出来的东西令人震惊：
//! 斑点、条纹、螺旋、脉冲波——生命的纹路。
//!
//! 这是图灵在 1952 年提出的形态发生理论的核心。
//! 豹纹、斑马纹、珊瑚的分支，都是这个方程的解。
//!
//! ∂U/∂t = Du·∇²U  -  U·V²  +  f·(1 - U)
//! ∂V/∂t = Dv·∇²V  +  U·V²  -  (f + k)·V

use core::{rule::Rule, field::{Field, ScalarField}, math::laplacian};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ReactionDiffusionState {
    pub u: ScalarField,
    pub v: ScalarField,
}

/// 反应扩散系统的物理参数
///
/// 改变这些参数，就是改变这个宇宙的物理常数。
/// 不同的 f 和 k 会让宇宙长出完全不同的东西。
#[derive(Clone, Serialize, Deserialize)]
pub struct ReactionDiffusion {
    /// U 的扩散速率
    pub du: f32,
    /// V 的扩散速率
    pub dv: f32,
    /// 补充速率 (feed rate)
    pub f: f32,
    /// 消耗速率 (kill rate)
    pub k: f32,
}

impl ReactionDiffusion {
    /// 默认参数 — 会长出移动的斑点
    pub fn default_spots() -> Self {
        Self { du: 0.2097, dv: 0.105, f: 0.036, k: 0.065 }
    }

    /// 会长出条纹
    pub fn stripes() -> Self {
        Self { du: 0.2097, dv: 0.105, f: 0.022, k: 0.051 }
    }

    /// 会长出螺旋
    pub fn spirals() -> Self {
        Self { du: 0.2097, dv: 0.105, f: 0.018, k: 0.051 }
    }
}

impl Rule for ReactionDiffusion {
    type State = ReactionDiffusionState;

    fn step(&self, state: &Self::State, dt: f32) -> Self::State {
        let w = state.u.width;
        let h = state.u.height;
        let mut next_u = state.u.clone();
        let mut next_v = state.v.clone();

        for y in 0..h {
            for x in 0..w {
                let u = *state.u.get(x, y);
                let v = *state.v.get(x, y);

                let lap_u = laplacian(&state.u.data, x, y, w, h);
                let lap_v = laplacian(&state.v.data, x, y, w, h);

                let reaction = u * v * v;

                let du = self.du * lap_u - reaction + self.f * (1.0 - u);
                let dv = self.dv * lap_v + reaction - (self.f + self.k) * v;

                next_u.set(x, y, (u + du * dt).clamp(0.0, 1.0));
                next_v.set(x, y, (v + dv * dt).clamp(0.0, 1.0));
            }
        }

        ReactionDiffusionState { u: next_u, v: next_v }
    }

    fn name(&self) -> &str {
        "reaction-diffusion"
    }
}
