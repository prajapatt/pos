use super::provider::{validate_request, ModelProvider, ModelRequest, ModelResponse, ProviderError};

pub struct OpenRouterConfig {
	pub endpoint: String,
	pub api_key_environment: String,
}

impl OpenRouterConfig {
	pub fn new(endpoint: impl Into<String>, api_key_environment: impl Into<String>) -> Result<Self, ProviderError> {
		let config = Self {
			endpoint: endpoint.into(),
			api_key_environment: api_key_environment.into(),
		};
		if !config.endpoint.starts_with("https://") {
			return Err(ProviderError::InvalidConfiguration("OpenRouter endpoint must use HTTPS"));
		}
		if config.api_key_environment.trim().is_empty() {
			return Err(ProviderError::InvalidConfiguration("API key environment name is empty"));
		}
		Ok(config)
	}
}

pub struct OpenRouterProvider {
	config: OpenRouterConfig,
}

impl OpenRouterProvider {
	pub fn new(config: OpenRouterConfig) -> Self {
		Self { config }
	}
}

impl ModelProvider for OpenRouterProvider {
	fn name(&self) -> &str {
		"openrouter"
	}

	fn generate(&mut self, request: &ModelRequest) -> Result<ModelResponse, ProviderError> {
		validate_request(request)?;
		if std::env::var_os(&self.config.api_key_environment).is_none() {
			return Err(ProviderError::Unavailable(format!(
				"missing API key environment variable {}",
				self.config.api_key_environment
			)));
		}
		Err(ProviderError::Unavailable(format!(
			"HTTP transport is not configured for {}",
			self.config.endpoint
		)))
	}
}
