#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessState {
	Created,
	Runnable,
	Running,
	Blocked,
	Exited(u64),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Process {
	pub id: u64,
	pub parent: Option<u64>,
	pub state: ProcessState,
}

impl Process {
	pub const fn new(id: u64, parent: Option<u64>) -> Self {
		Self {
			id,
			parent,
			state: ProcessState::Created,
		}
	}

	pub fn make_runnable(&mut self) {
		self.state = ProcessState::Runnable;
	}

	pub fn block(&mut self) {
		self.state = ProcessState::Blocked;
	}

	pub fn wake(&mut self) {
		self.state = ProcessState::Runnable;
	}

	pub fn exit(&mut self, code: u64) {
		self.state = ProcessState::Exited(code);
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessTableError {
	CapacityReached,
	DuplicateProcess,
	UnknownProcess,
	ProcessNotExited,
}

pub struct ProcessTable<const CAPACITY: usize> {
	processes: [Option<Process>; CAPACITY],
	cursor: usize,
	current: Option<u64>,
}

impl<const CAPACITY: usize> ProcessTable<CAPACITY> {
	pub const fn new() -> Self {
		Self {
			processes: [None; CAPACITY],
			cursor: 0,
			current: None,
		}
	}

	pub fn spawn(&mut self, process: Process) -> Result<(), ProcessTableError> {
		if self.find(process.id).is_some() {
			return Err(ProcessTableError::DuplicateProcess);
		}
		let slot = self
			.processes
			.iter_mut()
			.find(|entry| entry.is_none())
			.ok_or(ProcessTableError::CapacityReached)?;
		*slot = Some(process);
		Ok(())
	}

	pub fn state(&self, id: u64) -> Result<ProcessState, ProcessTableError> {
		self.find(id)
			.and_then(|index| self.processes[index].map(|process| process.state))
			.ok_or(ProcessTableError::UnknownProcess)
	}

	pub fn schedule_next(&mut self) -> Option<u64> {
		if let Some(current) = self.current.take() {
			if let Some(index) = self.find(current) {
				if let Some(process) = self.processes[index].as_mut() {
					if process.state == ProcessState::Running {
						process.state = ProcessState::Runnable;
					}
				}
				self.cursor = (index + 1) % CAPACITY.max(1);
			}
		}

		for offset in 0..CAPACITY {
			let index = (self.cursor + offset) % CAPACITY.max(1);
			if let Some(process) = self.processes[index].as_mut() {
				if matches!(process.state, ProcessState::Runnable | ProcessState::Created) {
					process.state = ProcessState::Running;
					self.current = Some(process.id);
					self.cursor = (index + 1) % CAPACITY.max(1);
					return Some(process.id);
				}
			}
		}
		None
	}

	pub fn block(&mut self, id: u64) -> Result<(), ProcessTableError> {
		let index = self.find(id).ok_or(ProcessTableError::UnknownProcess)?;
		if let Some(process) = self.processes[index].as_mut() {
			process.block();
		}
		if self.current == Some(id) {
			self.current = None;
		}
		Ok(())
	}

	pub fn wake(&mut self, id: u64) -> Result<(), ProcessTableError> {
		let index = self.find(id).ok_or(ProcessTableError::UnknownProcess)?;
		if let Some(process) = self.processes[index].as_mut() {
			process.wake();
		}
		Ok(())
	}

	pub fn exit(&mut self, id: u64, code: u64) -> Result<(), ProcessTableError> {
		let index = self.find(id).ok_or(ProcessTableError::UnknownProcess)?;
		if let Some(process) = self.processes[index].as_mut() {
			process.exit(code);
		}
		if self.current == Some(id) {
			self.current = None;
		}
		Ok(())
	}

	pub fn reap(&mut self, id: u64) -> Result<(), ProcessTableError> {
		let index = self.find(id).ok_or(ProcessTableError::UnknownProcess)?;
		match self.processes[index].map(|process| process.state) {
			Some(ProcessState::Exited(_)) => {
				self.processes[index] = None;
				if self.current == Some(id) {
					self.current = None;
				}
				Ok(())
			}
			_ => Err(ProcessTableError::ProcessNotExited),
		}
	}

	pub fn running(&self) -> Option<u64> {
		self.current
	}

	fn find(&self, id: u64) -> Option<usize> {
		self.processes
			.iter()
			.position(|entry| entry.map(|process| process.id) == Some(id))
	}
}

impl<const CAPACITY: usize> Default for ProcessTable<CAPACITY> {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::{Process, ProcessState, ProcessTable};

	#[test]
	fn spawns_and_schedules_processes() {
		let mut table = ProcessTable::<4>::new();
		table.spawn(Process::new(1, None)).unwrap();
		table.spawn(Process::new(2, Some(1))).unwrap();

		assert_eq!(table.schedule_next(), Some(1));
		assert_eq!(table.running(), Some(1));
		assert_eq!(table.state(1), Ok(ProcessState::Running));
		assert_eq!(table.schedule_next(), Some(2));
	}

	#[test]
	fn blocked_processes_can_be_woken_and_reaped() {
		let mut table = ProcessTable::<2>::new();
		table.spawn(Process::new(7, None)).unwrap();
		table.schedule_next();
		table.block(7).unwrap();
		assert_eq!(table.state(7), Ok(ProcessState::Blocked));
		table.wake(7).unwrap();
		assert_eq!(table.state(7), Ok(ProcessState::Runnable));
		table.exit(7, 0).unwrap();
		table.reap(7).unwrap();
		assert_eq!(table.state(7).is_err(), true);
	}
}
