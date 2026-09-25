use crate::agent::context::SystemContext;
use crate::agent::executor::{ToolRequest, ToolResult, ToolRuntime};

use super::{diagnostics, filesystem, processes, system_info};

pub struct HostRuntime;

impl HostRuntime {
	fn filesystem_root() -> Result<filesystem::FilesystemTool, String> {
		let root = std::env::var_os("CHUT_OS_FILESYSTEM_ROOT")
			.map(std::path::PathBuf::from)
			.unwrap_or(std::env::current_dir().map_err(|error| error.to_string())?);
		filesystem::FilesystemTool::new(root).map_err(|error| format!("filesystem unavailable: {error:?}"))
	}
}

impl ToolRuntime for HostRuntime {
	fn execute(&mut self, request: &ToolRequest, _context: &SystemContext) -> ToolResult {
		let outcome = match request.operation.as_str() {
			"system.info" => system_info::collect().map(|info| system_info::format(&info)),
			"process.list" => processes::list().map(|items| processes::format(&items)),
			"diagnostics.run" => diagnostics::collect(),
			"filesystem.read_text" => Self::filesystem_root()
				.and_then(|tool| tool.read_text(&request.argument).map_err(|error| format!("filesystem read failed: {error:?}"))),
			"filesystem.create_directory" => Self::filesystem_root()
				.and_then(|tool| tool.create_directory(&request.argument).map_err(|error| format!("filesystem create failed: {error:?}"))),
			_ => Err(format!("unsupported host operation: {}", request.operation)),
		};
		match outcome {
			Ok(detail) => ToolResult {
				success: true,
				verified: true,
				detail,
			},
			Err(detail) => ToolResult {
				success: false,
				verified: true,
				detail,
			},
		}
	}
}