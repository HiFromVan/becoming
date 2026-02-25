//! universe — 宇宙的状态与时间
//!
//! 物理规则是永恒的，但宇宙是有历史的。
//! 这里管理宇宙从初始条件出发，一步一步演化的过程。
//! 它也负责保存和加载宇宙的状态——
//! 因为一个值得观察的宇宙，应该能被记住。

pub mod state;
pub mod constants;
pub mod init;

pub use state::Universe;
pub use constants::PhysicalConstants;
