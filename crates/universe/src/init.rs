use core::field::{Field, ScalarField};
use physics::{ReactionDiffusion, reaction_diffusion::ReactionDiffusionState};
use crate::state::Universe;

/// 初始条件的构造器
///
/// 宇宙从哪里开始，决定了它能去哪里。
/// 但即使是完全相同的规则，不同的初始条件
/// 也会长出完全不同的宇宙。
pub struct Init;

impl Init {
    /// 均匀背景 + 中心扰动
    ///
    /// 最经典的初始条件：宇宙一开始几乎是均匀的，
    /// 只有中心有一点点不均匀。
    /// 然后看这个微小的扰动如何被放大，长成整个宇宙的结构。
    pub fn center_seed(width: u32, height: u32) -> ReactionDiffusionState {
        let mut u = ScalarField::filled(width, height, 1.0);
        let mut v = ScalarField::zeros(width, height);

        // 在中心放一粒种子
        let cx = width / 2;
        let cy = height / 2;
        let seed_size = (width.min(height) / 20).max(2);

        for dy in 0..seed_size {
            for dx in 0..seed_size {
                let x = cx.saturating_sub(seed_size / 2) + dx;
                let y = cy.saturating_sub(seed_size / 2) + dy;
                if x < width && y < height {
                    u.set(x, y, 0.5);
                    v.set(x, y, 0.25);
                }
            }
        }

        ReactionDiffusionState { u, v }
    }

    /// 随机噪声初始条件
    pub fn random_noise(width: u32, height: u32, seed: u64) -> ReactionDiffusionState {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut u = ScalarField::zeros(width, height);
        let mut v = ScalarField::zeros(width, height);

        for y in 0..height {
            for x in 0..width {
                let mut hasher = DefaultHasher::new();
                (seed, x, y, 0u8).hash(&mut hasher);
                let h = hasher.finish();
                let val_u = (h & 0xFFFF) as f32 / 65535.0;

                let mut hasher2 = DefaultHasher::new();
                (seed, x, y, 1u8).hash(&mut hasher2);
                let h2 = hasher2.finish();
                let val_v = (h2 & 0xFFFF) as f32 / 65535.0 * 0.1;

                u.set(x, y, 0.5 + val_u * 0.5);
                v.set(x, y, val_v);
            }
        }

        ReactionDiffusionState { u, v }
    }

    /// 创建一个默认的反应扩散宇宙
    pub fn default_universe(width: u32, height: u32) -> Universe<ReactionDiffusion> {
        let rule = ReactionDiffusion::default_spots();
        let state = Self::center_seed(width, height);
        Universe::new(rule, state, 1.0)
    }
}
