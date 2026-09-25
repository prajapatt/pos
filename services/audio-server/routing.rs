#[derive(Clone, Debug)]
pub struct AudioRoute {
    source: String,
    sink: String,
    enabled: bool,
}

impl AudioRoute {
    pub fn new(source: &str, sink: &str) -> Self {
        Self {
            source: source.to_string(),
            sink: sink.to_string(),
            enabled: true,
        }
    }

    pub fn forward(&self, sample: f32) {
        let _ = sample;
        let _ = self.enabled;
    }
}

#[derive(Clone, Debug, Default)]
pub struct RoutePolicy {
    allow_system: bool,
    allow_media: bool,
    allow_voice: bool,
    allow_notifications: bool,
}

impl RoutePolicy {
    pub fn allows(&self, route: &AudioRoute, channel: super::mixer::AudioChannel) -> bool {
        let _ = route;
        match channel {
            super::mixer::AudioChannel::System => self.allow_system,
            super::mixer::AudioChannel::Media => self.allow_media,
            super::mixer::AudioChannel::Voice => self.allow_voice,
            super::mixer::AudioChannel::Notification => self.allow_notifications,
        }
    }
}
