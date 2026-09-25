use super::approval::ApprovalToken;
use super::capability::{Capability, CapabilitySet, RiskLevel};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyDecision {
	Allow,
	RequireApproval,
	Deny,
}

pub struct Policy {
	granted: CapabilitySet,
}

impl Policy {
	pub const fn new(granted: CapabilitySet) -> Self {
		Self { granted }
	}

	pub fn evaluate(&self, capability: Capability, approval: Option<ApprovalToken>) -> PolicyDecision {
		if !self.granted.contains(capability) {
			return PolicyDecision::Deny;
		}
		match capability.risk() {
			RiskLevel::ReadOnly | RiskLevel::Mutating => PolicyDecision::Allow,
			RiskLevel::Dangerous => {
				if approval.is_some_and(|token| token.action().required_capability() == capability) {
					PolicyDecision::Allow
				} else {
					PolicyDecision::RequireApproval
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Policy, PolicyDecision};
	use crate::permissions::approval::ApprovalStore;
	use crate::permissions::capability::{Capability, CapabilitySet};
	use crate::permissions::dangerous_actions::DangerousAction;

	#[test]
	fn read_only_capability_is_allowed_when_granted() {
		let policy = Policy::new(CapabilitySet::with(Capability::SystemInfo));

		assert_eq!(policy.evaluate(Capability::SystemInfo, None), PolicyDecision::Allow);
		assert_eq!(policy.evaluate(Capability::NetworkRead, None), PolicyDecision::Deny);
	}

	#[test]
	fn dangerous_capability_requires_matching_single_use_approval() {
		let granted = CapabilitySet::with(Capability::FilesystemDelete);
		let policy = Policy::new(granted);
		let mut approvals = ApprovalStore::new();
		approvals.request(42, DangerousAction::DeleteFiles).unwrap();

		assert_eq!(
			policy.evaluate(Capability::FilesystemDelete, None),
			PolicyDecision::RequireApproval
		);
		let token = approvals.approve(42).unwrap();
		assert_eq!(
			policy.evaluate(Capability::FilesystemDelete, Some(token)),
			PolicyDecision::Allow
		);
		assert_eq!(approvals.approve(42), Err(crate::permissions::approval::ApprovalError::NoPendingRequest));
	}

	#[test]
	fn approval_cannot_be_reused_for_another_capability() {
		let policy = Policy::new(CapabilitySet::with(Capability::FilesystemDelete));
		let mut approvals = ApprovalStore::new();
		approvals.request(7, DangerousAction::DeleteFiles).unwrap();
		let token = approvals.approve(7).unwrap();

		assert_eq!(policy.evaluate(Capability::ProcessStop, Some(token)), PolicyDecision::Deny);
	}
}
