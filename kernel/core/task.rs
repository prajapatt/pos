#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskState {
	New,
	Ready,
	Running,
	Blocked,
	Exited,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Task {
	pub id: u64,
	pub name: &'static str,
	pub state: TaskState,
	pub priority: u8,
	pub stack_pointer: u64,
}

impl Task {
	pub const fn new(id: u64, name: &'static str) -> Self {
		Self {
			id,
			name,
			state: TaskState::New,
			priority: 128,
			stack_pointer: 0,
		}
	}

	pub fn set_state(&mut self, state: TaskState) {
		self.state = state;
	}
}

#[cfg(test)]
mod tests {
	use super::{Task, TaskState};

	#[test]
	fn task_starts_in_new_state_and_can_transition() {
		let mut task = Task::new(1, "init");
		assert_eq!(task.state, TaskState::New);
		task.set_state(TaskState::Ready);
		assert_eq!(task.state, TaskState::Ready);
	}
}
