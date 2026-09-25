use super::action::{Action, ActionError};
use super::workflow::{Workflow, WorkflowError};

pub const MAX_RECORDED_ACTIONS: usize = 32;

#[derive(Default)]
pub struct MacroRecorder {
	recording: bool,
	actions: Vec<Action>,
}

impl MacroRecorder {
	pub fn start(&mut self) {
		self.recording = true;
		self.actions.clear();
	}

	pub fn stop(&mut self, name: impl Into<String>) -> Result<Workflow, WorkflowError> {
		self.recording = false;
		Workflow::new(name, self.actions.clone())
	}

	pub fn record(&mut self, action: Action) -> Result<(), ActionError> {
		if self.recording && self.actions.len() < MAX_RECORDED_ACTIONS {
			self.actions.push(action);
		}
		Ok(())
	}

	pub fn is_recording(&self) -> bool {
		self.recording
	}
}
