use super::scheduler::{Scheduler, SchedulerError, TaskId};
use super::timer::Timer;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelState {
	Cold,
	Booting,
	Running,
	ShuttingDown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelError {
	NotRunning,
	Scheduler(SchedulerError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServiceDescriptor {
	pub name: &'static str,
	pub priority: u8,
}

pub struct Kernel<const TASK_CAPACITY: usize> {
	state: KernelState,
	ticks: u64,
	timer: Timer,
	scheduler: Scheduler<TASK_CAPACITY>,
	services: [Option<ServiceDescriptor>; 8],
	service_count: usize,
}

impl<const TASK_CAPACITY: usize> Kernel<TASK_CAPACITY> {
	pub const fn new() -> Self {
		Self {
			state: KernelState::Cold,
			ticks: 0,
			timer: Timer::new(1000),
			scheduler: Scheduler::new(),
			services: [None; 8],
			service_count: 0,
		}
	}

	pub fn boot(&mut self) {
		self.state = KernelState::Booting;
		self.timer.tick();
		self.state = KernelState::Running;
	}

	pub fn state(&self) -> KernelState {
		self.state
	}

	pub fn ticks(&self) -> u64 {
		self.ticks
	}

	pub fn timer_ticks(&self) -> u64 {
		self.timer.ticks()
	}

	pub fn register_service(&mut self, name: &'static str, priority: u8) -> Result<(), KernelError> {
		if self.service_count >= self.services.len() {
			return Err(KernelError::NotRunning);
		}
		self.services[self.service_count] = Some(ServiceDescriptor { name, priority });
		self.service_count += 1;
		Ok(())
	}

	pub fn service_count(&self) -> usize {
		self.service_count
	}

	pub fn tick(&mut self) -> Result<Option<TaskId>, KernelError> {
		self.require_running()?;
		self.ticks = self.ticks.saturating_add(1);
		self.timer.tick();
		Ok(self.scheduler.schedule_next())
	}

	pub fn spawn(&mut self, id: TaskId) -> Result<(), KernelError> {
		self.require_running()?;
		self.scheduler.spawn(id).map_err(KernelError::Scheduler)
	}

	pub fn block(&mut self, id: TaskId) -> Result<(), KernelError> {
		self.require_running()?;
		self.scheduler.block(id).map_err(KernelError::Scheduler)
	}

	pub fn wake(&mut self, id: TaskId) -> Result<(), KernelError> {
		self.require_running()?;
		self.scheduler.wake(id).map_err(KernelError::Scheduler)
	}

	pub fn exit(&mut self, id: TaskId) -> Result<(), KernelError> {
		self.require_running()?;
		self.scheduler.exit(id).map_err(KernelError::Scheduler)
	}

	pub fn reap(&mut self, id: TaskId) -> Result<(), KernelError> {
		self.require_running()?;
		self.scheduler.reap(id).map_err(KernelError::Scheduler)
	}

	pub fn running_task(&self) -> Option<TaskId> {
		self.scheduler.running()
	}

	fn require_running(&self) -> Result<(), KernelError> {
		if self.state == KernelState::Running {
			Ok(())
		} else {
			Err(KernelError::NotRunning)
		}
	}
}

impl<const TASK_CAPACITY: usize> Default for Kernel<TASK_CAPACITY> {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::{Kernel, KernelError, KernelState};

	#[test]
	fn kernel_rejects_work_before_boot() {
		let mut kernel = Kernel::<2>::new();

		assert_eq!(kernel.state(), KernelState::Cold);
		assert_eq!(kernel.spawn(1), Err(KernelError::NotRunning));
		assert_eq!(kernel.tick(), Err(KernelError::NotRunning));
	}

	#[test]
	fn kernel_tick_advances_scheduler_and_time() {
		let mut kernel = Kernel::<2>::new();
		kernel.boot();
		kernel.spawn(11).unwrap();
		kernel.spawn(22).unwrap();

		assert_eq!(kernel.tick(), Ok(Some(11)));
		assert_eq!(kernel.ticks(), 1);
		assert_eq!(kernel.timer_ticks(), 2);
		assert_eq!(kernel.running_task(), Some(11));
		assert_eq!(kernel.tick(), Ok(Some(22)));
		assert_eq!(kernel.ticks(), 2);
		assert_eq!(kernel.timer_ticks(), 3);
	}

	#[test]
	fn kernel_registers_services_with_priority() {
		let mut kernel = Kernel::<1>::new();
		kernel.boot();
		kernel.register_service("init", 10).unwrap();
		kernel.register_service("fs", 20).unwrap();
		assert_eq!(kernel.service_count(), 2);
	}

	#[test]
	fn kernel_forwards_task_lifecycle_to_scheduler() {
		let mut kernel = Kernel::<1>::new();
		kernel.boot();
		kernel.spawn(7).unwrap();
		kernel.exit(7).unwrap();
		kernel.reap(7).unwrap();
		kernel.spawn(8).unwrap();

		assert_eq!(kernel.tick(), Ok(Some(8)));
	}
}