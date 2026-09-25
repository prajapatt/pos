#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoiceIndicatorState {
	Idle,
	Listening,
	Processing,
	Speaking,
	Unavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoiceIndicator {
	state: VoiceIndicatorState,
}

impl VoiceIndicator {
	pub const fn new() -> Self { Self { state: VoiceIndicatorState::Idle } }
	pub const fn state(self) -> VoiceIndicatorState { self.state }
	pub fn set_state(&mut self, state: VoiceIndicatorState) { self.state = state; }
}

impl Default for VoiceIndicator {
	fn default() -> Self { Self::new() }
}
