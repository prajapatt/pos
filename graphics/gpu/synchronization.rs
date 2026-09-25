#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FenceState { Unsignaled, Signaled }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fence { state: FenceState }
impl Fence { pub const fn new() -> Self { Self { state: FenceState::Unsignaled } } pub fn signal(&mut self) { self.state = FenceState::Signaled; } pub fn reset(&mut self) { self.state = FenceState::Unsignaled; } pub const fn is_signaled(&self) -> bool { matches!(self.state, FenceState::Signaled) } }
