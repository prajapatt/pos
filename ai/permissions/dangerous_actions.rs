use super::capability::{Capability, RiskLevel};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DangerousAction {
	DeleteFiles,
	FormatStorage,
	ModifyBootConfiguration,
	ChangeSecuritySettings,
	DisableProtection,
	TerminateCriticalService,
	ModifyPrivilegedNetworkRules,
	InstallPrivilegedComponent,
}

impl DangerousAction {
	pub const fn required_capability(self) -> Capability {
		match self {
			Self::DeleteFiles => Capability::FilesystemDelete,
			Self::FormatStorage => Capability::DeviceControl,
			Self::ModifyBootConfiguration => Capability::SettingsWrite,
			Self::ChangeSecuritySettings | Self::DisableProtection => Capability::SettingsWrite,
			Self::TerminateCriticalService => Capability::ProcessStop,
			Self::ModifyPrivilegedNetworkRules => Capability::NetworkConfigure,
			Self::InstallPrivilegedComponent => Capability::PackageInstall,
		}
	}

	pub const fn risk(self) -> RiskLevel {
		RiskLevel::Dangerous
	}
}
