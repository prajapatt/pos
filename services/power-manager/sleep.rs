#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SleepState {
    Idle,
    Suspend,
    Hibernate,
    Shutdown,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SleepPolicy {
    allowed: bool,
}

impl SleepPolicy {
    pub fn can_enter(&self, state: SleepState) -> bool {
        match state {
            SleepState::Idle => true,
            SleepState::Suspend => self.allowed,
            SleepState::Hibernate => self.allowed,
            SleepState::Shutdown => false,
        }
    }
}
