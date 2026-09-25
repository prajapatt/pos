#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RollbackState {
    Ready,
    InProgress,
    Completed,
    Failed,
}

#[derive(Clone, Debug, Default)]
pub struct RollbackPlan {
    state: RollbackState,
}

impl RollbackPlan {
    pub fn prepare(&mut self) -> RollbackState {
        self.state = RollbackState::Ready;
        self.state
    }
}
