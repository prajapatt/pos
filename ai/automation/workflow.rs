use super::action::{Action, ActionError};

pub const MAX_WORKFLOW_ACTIONS: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Workflow {
	name: String,
	actions: Vec<Action>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkflowError {
	EmptyName,
	NoActions,
	TooManyActions,
	InvalidAction(ActionError),
}

impl Workflow {
	pub fn new(name: impl Into<String>, actions: Vec<Action>) -> Result<Self, WorkflowError> {
		let name = name.into();
		if name.trim().is_empty() {
			return Err(WorkflowError::EmptyName);
		}
		if actions.is_empty() {
			return Err(WorkflowError::NoActions);
		}
		if actions.len() > MAX_WORKFLOW_ACTIONS {
			return Err(WorkflowError::TooManyActions);
		}
		for action in &actions {
			if action.request.operation.trim().is_empty() {
				return Err(WorkflowError::InvalidAction(ActionError::EmptyOperation));
			}
		}
		Ok(Self { name, actions })
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn actions(&self) -> &[Action] {
		&self.actions
	}
}
