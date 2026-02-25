//! physics — 物理定律的实现
//!
//! 这里住着宇宙的规则。
//! 每一个模块都是一套不同的物理定律，一个不同的宇宙。
//! 它们共享同一种语言（core），但各自定义不同的现实。

pub mod reaction_diffusion;

pub use reaction_diffusion::ReactionDiffusion;
