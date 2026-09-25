pub struct Timer { ticks: u64, frequency_hz: u32 }
impl Timer {
	pub const fn new(frequency_hz: u32) -> Self { Self { ticks: 0, frequency_hz } }
	pub fn tick(&mut self) { self.ticks = self.ticks.saturating_add(1); }
	pub const fn ticks(&self) -> u64 { self.ticks }
	pub fn elapsed_ms(&self) -> u64 {
		if self.frequency_hz == 0 { 0 } else { self.ticks.saturating_mul(1000) / self.frequency_hz as u64 }
	}
}

#[cfg(test)]
mod tests {
	use super::Timer;

	#[test]
	fn timer_counts_ticks_and_reports_elapsed_time() {
		let mut timer = Timer::new(1000);
		for _ in 0..3 {
			timer.tick();
		}
		assert_eq!(timer.ticks(), 3);
		assert_eq!(timer.elapsed_ms(), 3);
	}

	#[test]
	fn timer_handles_zero_frequency_safely() {
		let mut timer = Timer::new(0);
		timer.tick();
		assert_eq!(timer.ticks(), 1);
		assert_eq!(timer.elapsed_ms(), 0);
	}
}
