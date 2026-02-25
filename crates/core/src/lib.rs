//! core — 宇宙的基本语言
//!
//! 这里定义的不是具体的物理，而是描述物理所需的抽象：
//! 什么是场，什么是规则，什么是状态，时间如何流动。
//! 所有具体的宇宙都从这里的语言中生长出来。

pub mod field;
pub mod rule;
pub mod math;

pub use field::{Field, ScalarField, VectorField};
pub use rule::Rule;
pub use math::Vec2;
