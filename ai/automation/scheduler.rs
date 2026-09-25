use super::workflow::Workflow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScheduledWorkflow {
	pub workflow: Workflow,
	pub due_at: u64,
	pub interval: Option<u64>,
}

#[derive(Default)]
pub struct Scheduler {
	now: u64,
	entries: Vec<ScheduledWorkflow>,
}

impl Scheduler {
	pub fn schedule(
		&mut self,
		workflow: Workflow,
		due_at: u64,
		interval: Option<u64>,
	) {
		self.entries.push(ScheduledWorkflow {
			workflow,
			due_at,
			interval: interval.filter(|value| *value != 0),
		});
	}

	pub fn advance_to(&mut self, timestamp: u64) -> Vec<Workflow> {
		if timestamp < self.now {
			return Vec::new();
		}
		self.now = timestamp;
		let mut due = Vec::new();
		for entry in &mut self.entries {
			if entry.due_at <= self.now {
				due.push(entry.workflow.clone());
				if let Some(interval) = entry.interval {
					while entry.due_at <= self.now {
						entry.due_at = entry.due_at.saturating_add(interval);
					}
				} else {
					entry.due_at = u64::MAX;
				}
			}
		}
		due
	}

	pub fn now(&self) -> u64 {
		self.now
	}
}

#[cfg(test)]
mod tests {
	use super::Scheduler;
	use crate::agent::executor::ToolRequest;
	use crate::permissions::capability::Capability;
use crate::automation::action::Action;
use crate::automation::workflow::Workflow;

	fn workflow() -> Workflow {
		Workflow::new(
			"system check",
			vec![Action::new(ToolRequest {
				capability: Capability::SystemInfo,
				operation: "system.info".into(),
				argument: String::new(),
			})
			.unwrap()],
		)
		.unwrap()
	}

	#[test]
	fn one_shot_and_repeating_workflows_fire_at_due_time() {
		let mut scheduler = Scheduler::default();
		scheduler.schedule(workflow(), 10, None);
		scheduler.schedule(workflow(), 5, Some(5));

		assert!(scheduler.advance_to(4).is_empty());
		assert_eq!(scheduler.advance_to(5).len(), 1);
		assert_eq!(scheduler.advance_to(10).len(), 2);
		assert_eq!(scheduler.advance_to(11).len(), 0);
	}
}
