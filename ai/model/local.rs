use std::process::Command;

use super::provider::{validate_request, ModelProvider, ModelRequest, ModelResponse, ProviderError};

pub struct LocalModelProvider {
	executable: String,
	arguments: Vec<String>,
}

impl LocalModelProvider {
	pub fn new(executable: impl Into<String>) -> Result<Self, ProviderError> {
		let executable = executable.into();
		if executable.trim().is_empty() {
			return Err(ProviderError::InvalidConfiguration("local executable is empty"));
		}
		Ok(Self {
			executable,
			arguments: Vec::new(),
		})
	}

	pub fn with_arguments(mut self, arguments: impl IntoIterator<Item = String>) -> Self {
		self.arguments.extend(arguments);
		self
	}
}

impl ModelProvider for LocalModelProvider {
	fn name(&self) -> &str {
		"local-process"
	}

	fn generate(&mut self, request: &ModelRequest) -> Result<ModelResponse, ProviderError> {
		validate_request(request)?;
		let output = Command::new(&self.executable)
			.args(&self.arguments)
			.arg(&request.prompt)
			.output()
			.map_err(|error| ProviderError::LaunchFailed(error.to_string()))?;
		if !output.status.success() {
			return Err(ProviderError::ProcessFailed(
				String::from_utf8_lossy(&output.stderr).trim().to_string(),
			));
		}
		Ok(ModelResponse {
			model: request.model.clone(),
			text: String::from_utf8_lossy(&output.stdout).trim().to_string(),
		})
	}
}
