//! Fixed-capacity round-robin scheduling policy.

pub type TaskId = u64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskState {
	Ready,
	Running,
	Blocked,
	Exited,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Task {
	pub id: TaskId,
	pub state: TaskState,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulerError {
	CapacityReached,
	DuplicateTask,
	TaskNotExited,
	UnknownTask,
}

pub struct Scheduler<const CAPACITY: usize> {
	tasks: [Option<Task>; CAPACITY],
	cursor: usize,
	running: Option<TaskId>,
}

impl<const CAPACITY: usize> Scheduler<CAPACITY> {
	pub const fn new() -> Self {
		Self {
			tasks: [None; CAPACITY],
			cursor: 0,
			running: None,
		}
	}

	pub fn spawn(&mut self, id: TaskId) -> Result<(), SchedulerError> {
		if self.find_index(id).is_some() {
			return Err(SchedulerError::DuplicateTask);
		}

		let slot = self
			.tasks
			.iter_mut()
			.find(|task| task.is_none())
			.ok_or(SchedulerError::CapacityReached)?;
		*slot = Some(Task {
			id,
			state: TaskState::Ready,
		});
		Ok(())
	}

	pub fn schedule_next(&mut self) -> Option<TaskId> {
		if let Some(running_id) = self.running.take() {
			if let Some(index) = self.find_index(running_id) {
				if let Some(task) = self.tasks[index].as_mut() {
					if task.state == TaskState::Running {
						task.state = TaskState::Ready;
					}
				}
				self.cursor = (index + 1) % CAPACITY.max(1);
			}
		}

		for offset in 0..CAPACITY {
			let index = (self.cursor + offset) % CAPACITY.max(1);
			if let Some(task) = self.tasks[index].as_mut() {
				if task.state == TaskState::Ready {
					task.state = TaskState::Running;
					self.running = Some(task.id);
					self.cursor = (index + 1) % CAPACITY.max(1);
					return self.running;
				}
			}
		}
		None
	}

	pub fn block(&mut self, id: TaskId) -> Result<(), SchedulerError> {
		self.set_state(id, TaskState::Blocked)
	}

	pub fn wake(&mut self, id: TaskId) -> Result<(), SchedulerError> {
		self.set_state(id, TaskState::Ready)
	}

	pub fn exit(&mut self, id: TaskId) -> Result<(), SchedulerError> {
		self.set_state(id, TaskState::Exited)
	}

	pub fn reap(&mut self, id: TaskId) -> Result<(), SchedulerError> {
		let index = self.find_index(id).ok_or(SchedulerError::UnknownTask)?;
		if self.tasks[index].map(|task| task.state) != Some(TaskState::Exited) {
			return Err(SchedulerError::TaskNotExited);
		}
		self.tasks[index] = None;
		if self.running == Some(id) {
			self.running = None;
		}
		if index < self.cursor || self.cursor >= CAPACITY {
			self.cursor = index;
		}
		Ok(())
	}

	pub fn state(&self, id: TaskId) -> Result<TaskState, SchedulerError> {
		self.find_index(id)
			.and_then(|index| self.tasks[index].map(|task| task.state))
			.ok_or(SchedulerError::UnknownTask)
	}

	pub fn running(&self) -> Option<TaskId> {
		self.running
	}

	fn set_state(&mut self, id: TaskId, state: TaskState) -> Result<(), SchedulerError> {
		let index = self.find_index(id).ok_or(SchedulerError::UnknownTask)?;
		if self.running == Some(id) && state != TaskState::Running {
			self.running = None;
		}
		if let Some(task) = self.tasks[index].as_mut() {
			task.state = state;
		}
		Ok(())
	}

	fn find_index(&self, id: TaskId) -> Option<usize> {
		self.tasks
			.iter()
			.position(|task| task.map(|task| task.id) == Some(id))
	}
}

impl<const CAPACITY: usize> Default for Scheduler<CAPACITY> {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::{Scheduler, SchedulerError, TaskState};

	#[test]
	fn schedules_ready_tasks_in_round_robin_order() {
		let mut scheduler = Scheduler::<3>::new();
		scheduler.spawn(10).unwrap();
		scheduler.spawn(20).unwrap();
		scheduler.spawn(30).unwrap();

		assert_eq!(scheduler.schedule_next(), Some(10));
		assert_eq!(scheduler.schedule_next(), Some(20));
		assert_eq!(scheduler.schedule_next(), Some(30));
		assert_eq!(scheduler.schedule_next(), Some(10));
	}

	#[test]
	fn blocked_tasks_are_skipped_until_woken() {
		let mut scheduler = Scheduler::<2>::new();
		scheduler.spawn(1).unwrap();
		scheduler.spawn(2).unwrap();
		scheduler.block(2).unwrap();

		assert_eq!(scheduler.schedule_next(), Some(1));
		assert_eq!(scheduler.schedule_next(), Some(1));
		assert_eq!(scheduler.state(2), Ok(TaskState::Blocked));

		scheduler.wake(2).unwrap();
		assert_eq!(scheduler.schedule_next(), Some(2));
	}

	#[test]
	fn rejects_duplicate_tasks_and_capacity_overflow() {
		let mut scheduler = Scheduler::<1>::new();
		scheduler.spawn(7).unwrap();

		assert_eq!(scheduler.spawn(7), Err(SchedulerError::DuplicateTask));
		assert_eq!(scheduler.spawn(8), Err(SchedulerError::CapacityReached));
	}

	#[test]
	fn exited_tasks_are_not_scheduled() {
		let mut scheduler = Scheduler::<2>::new();
		scheduler.spawn(1).unwrap();
		scheduler.spawn(2).unwrap();
		scheduler.exit(1).unwrap();

		assert_eq!(scheduler.schedule_next(), Some(2));
		assert_eq!(scheduler.schedule_next(), Some(2));
	}

	#[test]
	fn exited_task_slots_can_be_reused_after_reaping() {
		let mut scheduler = Scheduler::<1>::new();
		scheduler.spawn(1).unwrap();
		assert_eq!(scheduler.reap(1), Err(SchedulerError::TaskNotExited));

		scheduler.exit(1).unwrap();
		scheduler.reap(1).unwrap();
		scheduler.spawn(2).unwrap();

		assert_eq!(scheduler.schedule_next(), Some(2));
	}
}
