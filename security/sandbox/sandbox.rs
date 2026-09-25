#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandboxAction {
	ReadMemory,
	WriteMemory,
	SpawnProcess,
	OpenFile,
	UseNetwork,
	AccessDevice,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SandboxConfig {
	pub allow_network: bool,
	pub allow_filesystem: bool,
	pub allow_process_spawn: bool,
	pub allow_device_access: bool,
	pub max_threads: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandboxDecision {
	Allowed,
	Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SandboxRuntime {
	pub config: SandboxConfig,
	pub active: bool,
}

impl SandboxRuntime {
	pub const fn new(config: SandboxConfig) -> Self {
		Self {
			config,
			active: true,
		}
	}

	pub fn evaluate(&self, action: SandboxAction) -> SandboxDecision {
		match action {
			SandboxAction::UseNetwork if !self.config.allow_network => SandboxDecision::Denied,
			SandboxAction::OpenFile if !self.config.allow_filesystem => SandboxDecision::Denied,
			SandboxAction::SpawnProcess if !self.config.allow_process_spawn => SandboxDecision::Denied,
			SandboxAction::AccessDevice if !self.config.allow_device_access => SandboxDecision::Denied,
			_ => SandboxDecision::Allowed,
		}
	}

	pub fn can_execute(&self, threads: u32) -> bool {
		self.active && threads <= self.config.max_threads
	}
}
