use super::audio_pipeline::AudioFrame;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpeechError { Unavailable(String), InvalidAudio(&'static str) }

pub trait SpeechToText {
	fn transcribe(&mut self, frame: &AudioFrame) -> Result<String, SpeechError>;
}

pub struct UnavailableSpeechRecognizer { reason: String }

impl UnavailableSpeechRecognizer {
	pub fn new(reason: impl Into<String>) -> Self { Self { reason: reason.into() } }
}

impl SpeechToText for UnavailableSpeechRecognizer {
	fn transcribe(&mut self, frame: &AudioFrame) -> Result<String, SpeechError> {
		if frame.samples.is_empty() { return Err(SpeechError::InvalidAudio("empty frame")); }
		Err(SpeechError::Unavailable(self.reason.clone()))
	}
}
