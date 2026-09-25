use crate::permissions::approval::ApprovalToken;

use super::context::SystemContext;
use super::executor::{ExecutionError, Executor, ToolResult, ToolRuntime};
use super::memory::ConversationMemory;
use super::planner::Planner;
use super::reasoning::classify;

#[derive(Debug, Eq, PartialEq)]
pub enum AgentError {
	UnknownIntent,
	Execution(ExecutionError),
}

pub struct Agent<R> {
	context: SystemContext,
	memory: ConversationMemory,
	executor: Executor<R>,
}

impl<R: ToolRuntime> Agent<R> {
	pub const fn new(context: SystemContext, executor: Executor<R>) -> Self {
		Self {
			context,
			memory: ConversationMemory::new(),
			executor,
		}
	}

	pub fn handle(&mut self, input: &str, approval: Option<ApprovalToken>) -> Result<ToolResult, AgentError> {
		self.memory.push("user", input);
		let intent = classify(input);
		let plan = Planner::create(intent).ok_or(AgentError::UnknownIntent)?;
		let step = plan.steps.first().ok_or(AgentError::UnknownIntent)?;
		let result = self
			executor
			.execute(&step.request, &self.context, approval)
			.map_err(AgentError::Execution)?;
		self.memory.push("system", &result.detail);
		Ok(result)
	}

	pub fn context(&self) -> &SystemContext {
		&self.context
	}

	pub fn memory(&self) -> &ConversationMemory {
		&self.memory
	}
}

#[cfg(test)]
mod tests {
	use super::{Agent, AgentError};
	use crate::agent::context::SystemContext;
	use crate::agent::executor::{ExecutionError, Executor, ToolRequest, ToolResult, ToolRuntime};
	use crate::permissions::approval::ApprovalStore;
	use crate::permissions::capability::{Capability, CapabilitySet};
	use crate::permissions::dangerous_actions::DangerousAction;
	use crate::permissions::policy::Policy;

	struct TestRuntime {
		result: ToolResult,
		last_operation: Option<String>,
	}

	impl ToolRuntime for TestRuntime {
		fn execute(&mut self, request: &ToolRequest, _context: &SystemContext) -> ToolResult {
			self.last_operation = Some(request.operation.clone());
			self.result.clone()
		}
	}

	fn agent_with(capability: Capability, result: ToolResult) -> Agent<TestRuntime> {
		let runtime = TestRuntime {
			result,
			last_operation: None,
		};
		Agent::new(
			SystemContext::new("tester"),
			Executor::new(runtime, Policy::new(CapabilitySet::with(capability))),
		)
	}

	#[test]
	fn verified_read_operation_is_returned_and_recorded() {
		let mut agent = agent_with(
			Capability::SystemInfo,
			ToolResult {
				success: true,
				verified: true,
				detail: "system information collected".into(),
			},
		);

		assert!(agent.handle("show system info", None).is_ok());
		assert_eq!(agent.memory().messages().len(), 2);
	}

	#[test]
	fn unverified_runtime_result_is_never_reported_as_success() {
		let mut agent = agent_with(
			Capability::SystemInfo,
			ToolResult {
				success: true,
				verified: false,
				detail: "model guessed the result".into(),
			},
		);

		assert_eq!(
			agent.handle("show system info", None),
			Err(AgentError::Execution(ExecutionError::Unverified))
		);
	}

	#[test]
	fn dangerous_request_needs_matching_approval() {
		let mut agent = agent_with(
			Capability::FilesystemDelete,
			ToolResult {
				success: true,
				verified: true,
				detail: "deleted".into(),
			},
		);
		assert_eq!(
			agent.handle("delete files /tmp/cache", None),
			Err(AgentError::Execution(ExecutionError::ApprovalRequired))
		);

		let mut approvals = ApprovalStore::new();
		approvals.request(1, DangerousAction::DeleteFiles).unwrap();
		let token = approvals.approve(1).unwrap();
		assert!(agent.handle("delete files /tmp/cache", Some(token)).is_ok());
}
*** Add File: a:\abhishekprajapatt\Systems-Engineer\pos\ai\agent\main.rs
use super::agent::Agent;
use super::executor::{ToolResult, ToolRuntime};
use crate::permissions::approval::ApprovalToken;

pub fn handle_input<R: ToolRuntime>(
	agent: &mut Agent<R>,
	input: &str,
	approval: Option<ApprovalToken>,
) -> Result<ToolResult, super::agent::AgentError> {
	agent.handle(input, approval)
}
