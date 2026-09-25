use super::activity::ActivityLog;
use super::chat::ChatSession;
use super::command_bar::CommandBar;
use super::voice_indicator::VoiceIndicator;

pub struct AiPanel {
	pub chat: ChatSession,
	pub activity: ActivityLog,
	pub command_bar: Option<CommandBar>,
	pub voice: VoiceIndicator,
}

impl AiPanel {
	pub fn new() -> Self {
		Self {
			chat: ChatSession::default(),
			activity: ActivityLog::default(),
			command_bar: None,
			voice: VoiceIndicator::new(),
		}
	}

	pub fn open_command_bar(&mut self) { self.command_bar = Some(CommandBar::open()); }
}

impl Default for AiPanel {
	fn default() -> Self { Self::new() }
}
