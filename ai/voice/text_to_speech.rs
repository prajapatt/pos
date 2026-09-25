#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpeechOutputError { Unavailable(String), EmptyText }

pub trait TextToSpeech {
	fn speak(&mut self, text: &str) -> Result<(), SpeechOutputError>;
}

pub struct UnavailableSpeaker { reason: String }

impl UnavailableSpeaker {
	pub fn new(reason: impl Into<String>) -> Self { Self { reason: reason.into() } }
}

impl TextToSpeech for UnavailableSpeaker {
	fn speak(&mut self, text: &str) -> Result<(), SpeechOutputError> {
		if text.trim().is_empty() { return Err(SpeechOutputError::EmptyText); }
		Err(SpeechOutputError::Unavailable(self.reason.clone()))
	}
}
