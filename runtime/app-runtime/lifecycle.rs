#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleEvent {
    Created,
    Started,
    Paused,
    Resumed,
    Stopped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LifecycleState {
    pub event: LifecycleEvent,
    pub version: u32,
}

impl LifecycleState {
    pub const fn new() -> Self {
        Self {
            event: LifecycleEvent::Created,
            version: 1,
        }
    }

    pub fn transition(&mut self, next: LifecycleEvent) {
        self.event = next;
        self.version = self.version.saturating_add(1);
    }
}

impl Default for LifecycleState {
    fn default() -> Self {
        Self::new()
    }
}
