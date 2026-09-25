use super::background_services::BackgroundPolicy;
use super::gpu_priority::GpuPolicy;
use super::memory::MemoryPolicy;
use super::scheduler::SchedulerPolicy;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameMode { Disabled, Enabled }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GamePolicy { pub scheduler: SchedulerPolicy, pub memory: MemoryPolicy, pub gpu: GpuPolicy, pub background: BackgroundPolicy }

pub struct GameModeManager { mode: GameMode, policy: GamePolicy }

impl GameModeManager {
	pub fn new() -> Self { Self { mode: GameMode::Disabled, policy: GamePolicy { scheduler: SchedulerPolicy::balanced(), memory: MemoryPolicy::balanced(), gpu: GpuPolicy::balanced(), background: BackgroundPolicy::normal() } } }
	pub fn enable(&mut self, memory_reserve_bytes: u64) { self.mode = GameMode::Enabled; self.policy = GamePolicy { scheduler: SchedulerPolicy::gaming(), memory: MemoryPolicy::gaming(memory_reserve_bytes), gpu: GpuPolicy::gaming(), background: BackgroundPolicy::gaming() }; }
	pub fn disable(&mut self) { self.mode = GameMode::Disabled; self.policy = GamePolicy { scheduler: SchedulerPolicy::balanced(), memory: MemoryPolicy::balanced(), gpu: GpuPolicy::balanced(), background: BackgroundPolicy::normal() }; }
	pub const fn mode(&self) -> GameMode { self.mode }
	pub const fn policy(&self) -> GamePolicy { self.policy }
}
impl Default for GameModeManager { fn default() -> Self { Self::new() } }
