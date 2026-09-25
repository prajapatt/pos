use crate::agent::executor::ToolRequest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Action {
	pub request: ToolRequest,
}

impl Action {
	pub fn new(request: ToolRequest) -> Result<Self, ActionError> {
		if request.operation.trim().is_empty() {
			return Err(ActionError::EmptyOperation);
		}
		Ok(Self { request })
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
	EmptyOperation,
}
