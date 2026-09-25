use std::env;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemInfo {
	pub operating_system: String,
	pub architecture: String,
	pub executable: String,
}

pub fn collect() -> Result<SystemInfo, String> {
	let executable = env::current_exe()
		.map_err(|error| format!("current executable unavailable: {error}"))?
		.display()
		.to_string();
	Ok(SystemInfo {
		operating_system: env::consts::OS.to_string(),
		architecture: env::consts::ARCH.to_string(),
		executable,
	})
}

pub fn format(info: &SystemInfo) -> String {
	format!(
		"os={} arch={} executable={}",
		info.operating_system, info.architecture, info.executable
	)
}
