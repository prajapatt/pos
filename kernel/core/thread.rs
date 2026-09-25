#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadState {
	Created,
	Ready,
	Running,
	Blocked,
	Finished,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Thread {
	pub id: u64,
	pub task_id: u64,
	pub state: ThreadState,
	pub cpu_id: u32,
	pub stack_pointer: u64,
}

impl Thread {
	pub const fn new(id: u64, task_id: u64, cpu_id: u32) -> Self {
		Self {
			id,
			task_id,
			state: ThreadState::Created,
			cpu_id,
			stack_pointer: 0,
		}
	}

	pub fn set_state(&mut self, state: ThreadState) {
		self.state = state;
	}
}

#[cfg(test)]
mod tests {
	use super::{Thread, ThreadState};

	#[test]
	fn thread_starts_created_and_can_run() {
		let mut thread = Thread::new(7, 3, 0);
		assert_eq!(thread.state, ThreadState::Created);
		thread.set_state(ThreadState::Ready);
		assert_eq!(thread.state, ThreadState::Ready);
	}
}
