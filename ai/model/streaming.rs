use super::provider::{ModelProvider, ModelRequest, ProviderError};

pub trait TokenSink {
	fn push(&mut self, token: &str);
}

pub fn generate_to_sink<P: ModelProvider, S: TokenSink>(
	provider: &mut P,
	request: &ModelRequest,
	sink: &mut S,
) -> Result<(), ProviderError> {
	let response = provider.generate(request)?;
	for token in response.text.split_whitespace() {
		sink.push(token);
	}
	Ok(())
}
