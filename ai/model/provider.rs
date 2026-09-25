#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelRequest {
	pub model: String,
	pub prompt: String,
	pub max_tokens: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelResponse {
	pub model: String,
	pub text: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderError {
	InvalidRequest(&'static str),
	Unavailable(String),
	LaunchFailed(String),
	ProcessFailed(String),
	InvalidConfiguration(&'static str),
}

pub trait ModelProvider {
	fn name(&self) -> &str;
	fn generate(&mut self, request: &ModelRequest) -> Result<ModelResponse, ProviderError>;
}

pub fn validate_request(request: &ModelRequest) -> Result<(), ProviderError> {
	if request.model.trim().is_empty() {
		return Err(ProviderError::InvalidRequest("model is empty"));
	}
	if request.prompt.trim().is_empty() {
		return Err(ProviderError::InvalidRequest("prompt is empty"));
	}
	if request.max_tokens == 0 {
		return Err(ProviderError::InvalidRequest("max_tokens must be non-zero"));
	}
	Ok(())
}
