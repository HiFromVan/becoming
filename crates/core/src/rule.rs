/// 规则 — 宇宙的物理定律
///
/// 规则不关心宇宙现在是什么样子，
/// 它只知道：给定当前的状态，下一刻应该是什么。
/// 这就是物理定律的全部。
pub trait Rule: Send + Sync {
    type State: Clone + Send + Sync;

    /// 将规则应用一步，dt 是时间步长
    fn step(&self, state: &Self::State, dt: f32) -> Self::State;

    /// 规则的名字，用于日志和保存
    fn name(&self) -> &str;
}
