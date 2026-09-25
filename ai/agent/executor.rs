use super::context::SystemContext;
use crate::permissions::approval::ApprovalToken;
use crate::permissions::capability::Capability;
use crate::permissions::policy::{Policy, PolicyDecision};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolRequest {
	pub capability: Capability,
	pub operation: String,
	pub argument: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolResult {
	pub success: bool,
	pub verified: bool,
	pub detail: String,
}

pub trait ToolRuntime {
	fn execute(&mut self, request: &ToolRequest, context: &SystemContext) -> ToolResult;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionError {
	Denied,
	ApprovalRequired,
	Unverified,
	RuntimeFailed,
}

pub struct Executor<R> {
	runtime: R,
	policy: Policy,
}

impl<R: ToolRuntime> Executor<R> {
	pub const fn new(runtime: R, policy: Policy) -> Self {
		Self { runtime, policy }
	}

	pub fn execute(
		&mut self,
		request: &ToolRequest,
		context: &SystemContext,
		approval: Option<ApprovalToken>,
	) -> Result<ToolResult, ExecutionError> {
		match self.policy.evaluate(request.capability, approval) {
			PolicyDecision::Deny => Err(ExecutionError::Denied),
			PolicyDecision::RequireApproval => Err(ExecutionError::ApprovalRequired),
			PolicyDecision::Allow => {
				let result = self.runtime.execute(request, context);
				if !result.verified {
					return Err(ExecutionError::Unverified);
				}
				if !result.success {
					return Err(ExecutionError::RuntimeFailed);
				}
				Ok(result)
			}
		}
	}
}
