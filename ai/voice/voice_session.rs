use super::audio_pipeline::AudioFrame;
use super::microphone::{Microphone, MicrophoneError};
use super::speech_to_text::{SpeechError, SpeechToText};
use super::wake_word::VoiceActivityDetector;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoiceSessionState { Idle, Listening, Recognizing, Unavailable }

pub struct VoiceSession<M, S> {
	microphone: M,
	stt: S,
	vad: VoiceActivityDetector,
	state: VoiceSessionState,
}

impl<M: Microphone, S: SpeechToText> VoiceSession<M, S> {
	pub fn new(microphone: M, stt: S, vad: VoiceActivityDetector) -> Self {
		Self { microphone, stt, vad, state: VoiceSessionState::Idle }
	}

	pub fn state(&self) -> VoiceSessionState { self.state }

	pub fn poll(&mut self) -> Result<Option<String>, VoiceSessionError> {
		self.state = VoiceSessionState::Listening;
		let frame = match self.microphone.next_frame() {
			Ok(frame) => frame,
			Err(error) => {
				self.state = VoiceSessionState::Unavailable;
				return Err(VoiceSessionError::Microphone(error));
			}
		};
		if !self.vad.is_speech(&frame) { return Ok(None); }
		self.state = VoiceSessionState::Recognizing;
		match self.stt.transcribe(&frame) {
			Ok(text) => Ok(Some(text)),
			Err(error) => {
				self.state = VoiceSessionState::Unavailable;
				Err(VoiceSessionError::Speech(error))
			}
		}
	}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VoiceSessionError { Microphone(MicrophoneError), Speech(SpeechError) }

pub fn validate_frame(frame: &AudioFrame) -> bool { frame.sample_rate >= 8000 && frame.channels <= 2 }
