#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiskLevel {
	ReadOnly,
	Mutating,
	Dangerous,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Capability {
	FilesystemRead = 0,
	FilesystemWrite = 1,
	FilesystemDelete = 2,
	ProcessList = 3,
	ProcessStart = 4,
	ProcessStop = 5,
	ApplicationLaunch = 6,
	SettingsRead = 7,
	SettingsWrite = 8,
	NetworkRead = 9,
	NetworkConfigure = 10,
	DeviceInspect = 11,
	DeviceControl = 12,
	DisplayControl = 13,
	AudioControl = 14,
	PackageInstall = 15,
	DiagnosticsRun = 16,
	SystemInfo = 17,
}

impl Capability {
	pub const fn name(self) -> &'static str {
		match self {
			Self::FilesystemRead => "filesystem.read",
			Self::FilesystemWrite => "filesystem.write",
			Self::FilesystemDelete => "filesystem.delete",
			Self::ProcessList => "process.list",
			Self::ProcessStart => "process.start",
			Self::ProcessStop => "process.stop",
			Self::ApplicationLaunch => "application.launch",
			Self::SettingsRead => "settings.read",
			Self::SettingsWrite => "settings.write",
			Self::NetworkRead => "network.read",
			Self::NetworkConfigure => "network.configure",
			Self::DeviceInspect => "device.inspect",
			Self::DeviceControl => "device.control",
			Self::DisplayControl => "display.control",
			Self::AudioControl => "audio.control",
			Self::PackageInstall => "package.install",
			Self::DiagnosticsRun => "diagnostics.run",
			Self::SystemInfo => "system.info",
		}
	}

	pub const fn risk(self) -> RiskLevel {
		match self {
			Self::FilesystemRead | Self::ProcessList | Self::SettingsRead | Self::NetworkRead |
			Self::DeviceInspect | Self::DiagnosticsRun | Self::SystemInfo => RiskLevel::ReadOnly,
			Self::FilesystemDelete | Self::ProcessStop | Self::NetworkConfigure |
			Self::PackageInstall => RiskLevel::Dangerous,
			_ => RiskLevel::Mutating,
		}
	}

	pub const fn mask(self) -> u64 {
		1u64 << (self as u8)
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapabilitySet {
	bits: u64,
}

impl CapabilitySet {
	pub const fn empty() -> Self {
		Self { bits: 0 }
	}

	pub const fn with(capability: Capability) -> Self {
		Self { bits: capability.mask() }
	}

	pub fn grant(&mut self, capability: Capability) {
		self.bits |= capability.mask();
	}

	pub fn revoke(&mut self, capability: Capability) {
		self.bits &= !capability.mask();
	}

	pub const fn contains(self, capability: Capability) -> bool {
		self.bits & capability.mask() != 0
	}
}
