use crate::permissions::capability::Capability;

use super::executor::ToolRequest;
use super::reasoning::Intent;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanStep {
	pub request: ToolRequest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
	pub steps: Vec<PlanStep>,
}

pub struct Planner;

impl Planner {
	pub fn create(intent: Intent) -> Option<Plan> {
		let request = match intent {
			Intent::ShowSystemInfo => ToolRequest {
				capability: Capability::SystemInfo,
				operation: "system.info".into(),
				argument: String::new(),
			},
			Intent::RunDiagnostics => ToolRequest {
				capability: Capability::DiagnosticsRun,
				operation: "diagnostics.run".into(),
				argument: String::new(),
			},
			Intent::ListProcesses => ToolRequest {
				capability: Capability::ProcessList,
				operation: "process.list".into(),
				argument: String::new(),
			},
			Intent::ReadFile(path) => ToolRequest {
				capability: Capability::FilesystemRead,
				operation: "filesystem.read_text".into(),
				argument: path,
			},
			Intent::CreateFolder(path) => ToolRequest {
				capability: Capability::FilesystemWrite,
				operation: "filesystem.create_directory".into(),
				argument: path,
			},
			Intent::DeleteFiles(path) => ToolRequest {
				capability: Capability::FilesystemDelete,
				operation: "filesystem.delete".into(),
				argument: path,
			},
			Intent::Unknown => return None,
		};
		Some(Plan { steps: vec![PlanStep { request }] })
	}
}
