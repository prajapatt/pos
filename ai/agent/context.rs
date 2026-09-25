#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SystemContext {
	pub current_user: String,
	pub active_application: Option<String>,
	pub active_window: Option<String>,
	pub memory_used_bytes: u64,
	pub running_processes: u32,
	pub network_online: bool,
}

impl SystemContext {
	pub fn new(current_user: impl Into<String>) -> Self {
		Self {
			current_user: current_user.into(),
			..Self::default()
		}
	}

	pub fn set_active_window(
		&mut self,
		application: impl Into<String>,
		window: impl Into<String>,
	) {
		self.active_application = Some(application.into());
		self.active_window = Some(window.into());
	}
}
