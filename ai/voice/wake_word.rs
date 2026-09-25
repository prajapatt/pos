use super::audio_pipeline::AudioFrame;

pub struct VoiceActivityDetector { threshold: f32 }

impl VoiceActivityDetector {
	pub const fn new(threshold: f32) -> Self { Self { threshold } }

	pub fn is_speech(&self, frame: &AudioFrame) -> bool { frame.rms() >= self.threshold }
}
