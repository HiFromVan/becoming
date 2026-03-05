//! 一个粒子在引力下运动 — Verlet 积分
//!
//! Verlet 积分的能量守恒性质让轨道保持稳定。
//! 粒子从 (100, 0) 出发，初速度 (0, 3)，
//! 应该画出一个封闭的椭圆。

use physics::{Gravity, ParticleState};
use core::rule::Rule;

fn main() {
    let rule = Gravity::new(1.0, 1000.0);
    let dt = 0.1;

    // 初始位置和速度
    let x0 = 100.0_f32;
    let y0 = 0.0_f32;
    let vx0 = 0.0_f32;
    let vy0 = 3.0_f32;

    // Verlet 需要"上一步"位置：用欧拉反推一步
    // prev = x0 - v*dt（相当于 t=-dt 时刻的位置）
    let state = ParticleState {
        x: x0,
        y: y0,
        vx: vx0,
        vy: vy0,
        prev_x: x0 - vx0 * dt,
        prev_y: y0 - vy0 * dt,
    };

    let mut state = state;

    println!("step, x, y");
    for i in 0..2000 {
        if i % 20 == 0 {
            println!("{}, {:.3}, {:.3}", i, state.x, state.y);
        }
        state = rule.step(&state, dt);
    }
}
