use super::audio_pipeline::AudioFrame;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MicrophoneError { Unavailable(String) }

pub trait Microphone {
	fn next_frame(&mut self) -> Result<AudioFrame, MicrophoneError>;
}

pub struct UnavailableMicrophone { reason: String }

impl UnavailableMicrophone {
	pub fn new(reason: impl Into<String>) -> Self { Self { reason: reason.into() } }
}

impl Microphone for UnavailableMicrophone {
	fn next_frame(&mut self) -> Result<AudioFrame, MicrophoneError> {
		Err(MicrophoneError::Unavailable(self.reason.clone()))
	}
}
