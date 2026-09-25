#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Intent {
	ShowSystemInfo,
	RunDiagnostics,
	ListProcesses,
	ReadFile(String),
	CreateFolder(String),
	DeleteFiles(String),
	Unknown,
}

pub fn classify(input: &str) -> Intent {
	let normalized = input.trim().to_ascii_lowercase();
	if normalized.contains("system info") || normalized.contains("cpu usage") {
		return Intent::ShowSystemInfo;
	}
	if normalized.contains("diagnostics") || normalized.contains("system health") {
		return Intent::RunDiagnostics;
	}
	if normalized == "ps" || normalized.contains("list processes") {
		return Intent::ListProcesses;
	}
	if let Some(path) = normalized.strip_prefix("read file ") {
		if !path.trim().is_empty() {
			return Intent::ReadFile(path.trim().to_string());
		}
	}
	if let Some(path) = normalized.strip_prefix("create folder ") {
		if !path.trim().is_empty() {
			return Intent::CreateFolder(path.trim().to_string());
		}
	}
	if let Some(path) = normalized.strip_prefix("delete files ") {
		if !path.trim().is_empty() {
			return Intent::DeleteFiles(path.trim().to_string());
		}
	}
	Intent::Unknown
}
