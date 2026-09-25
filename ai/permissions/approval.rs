use super::dangerous_actions::DangerousAction;

#[derive(Debug, Eq, PartialEq)]
pub struct ApprovalToken {
	request_id: u64,
	action: DangerousAction,
}

impl ApprovalToken {
	pub const fn request_id(self) -> u64 {
		self.request_id
	}

	pub const fn action(self) -> DangerousAction {
		self.action
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApprovalError {
	AlreadyPending,
	NoPendingRequest,
	WrongRequest,
}

pub struct ApprovalStore {
	pending: Option<ApprovalToken>,
}

impl ApprovalStore {
	pub const fn new() -> Self {
		Self { pending: None }
	}

	pub fn request(&mut self, request_id: u64, action: DangerousAction) -> Result<(), ApprovalError> {
		if self.pending.is_some() {
			return Err(ApprovalError::AlreadyPending);
		}
		self.pending = Some(ApprovalToken { request_id, action });
		Ok(())
	}

	pub fn approve(&mut self, request_id: u64) -> Result<ApprovalToken, ApprovalError> {
		let token = self.pending.ok_or(ApprovalError::NoPendingRequest)?;
		if token.request_id != request_id {
			return Err(ApprovalError::WrongRequest);
		}
		self.pending = None;
		Ok(token)
	}

	pub fn cancel(&mut self, request_id: u64) -> Result<(), ApprovalError> {
		let token = self.pending.ok_or(ApprovalError::NoPendingRequest)?;
		if token.request_id != request_id {
			return Err(ApprovalError::WrongRequest);
		}
		self.pending = None;
		Ok(())
	}
}

impl Default for ApprovalStore {
	fn default() -> Self {
		Self::new()
	}
}
