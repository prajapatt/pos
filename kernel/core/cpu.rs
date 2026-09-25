#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpuCore {
	pub id: u32,
	pub online: bool,
	pub current_task: Option<u64>,
}

impl CpuCore {
	pub const fn new(id: u32) -> Self {
		Self {
			id,
			online: true,
			current_task: None,
		}
	}

	pub fn set_task(&mut self, task_id: Option<u64>) {
		self.current_task = task_id;
	}
}

#[cfg(test)]
mod tests {
	use super::CpuCore;

	#[test]
	fn cpu_core_tracks_active_task() {
		let mut core = CpuCore::new(0);
		assert!(core.online);
		assert_eq!(core.current_task, None);
		core.set_task(Some(42));
		assert_eq!(core.current_task, Some(42));
	}
}
