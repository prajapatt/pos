#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AudioChannel {
    System,
    Media,
    Voice,
    Notification,
}

#[derive(Clone, Copy, Debug)]
pub struct MixerConfig {
    pub master_gain: f32,
    pub system_gain: f32,
    pub media_gain: f32,
    pub voice_gain: f32,
    pub notification_gain: f32,
}

impl Default for MixerConfig {
    fn default() -> Self {
        Self {
            master_gain: 1.0,
            system_gain: 0.9,
            media_gain: 1.0,
            voice_gain: 1.25,
            notification_gain: 0.8,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AudioMixer {
    config: MixerConfig,
}

impl AudioMixer {
    pub fn new(config: MixerConfig) -> Self {
        Self { config }
    }

    pub fn apply_gain(&self, channel: AudioChannel, sample: f32) -> f32 {
        let gain = match channel {
            AudioChannel::System => self.config.system_gain,
            AudioChannel::Media => self.config.media_gain,
            AudioChannel::Voice => self.config.voice_gain,
            AudioChannel::Notification => self.config.notification_gain,
        };
        (sample * self.config.master_gain * gain).clamp(-1.0, 1.0)
    }
}
