mod mixer;
mod routing;

use mixer::{AudioChannel, AudioMixer, MixerConfig};
use routing::{AudioRoute, RoutePolicy};

#[derive(Clone, Debug)]
pub struct AudioServer {
    mixer: AudioMixer,
    routes: Vec<AudioRoute>,
    policy: RoutePolicy,
}

impl AudioServer {
    pub fn new() -> Self {
        Self {
            mixer: AudioMixer::new(MixerConfig::default()),
            routes: Vec::new(),
            policy: RoutePolicy::default(),
        }
    }

    pub fn register_route(&mut self, route: AudioRoute) {
        self.routes.push(route);
    }

    pub fn process_frame(&mut self, channel: AudioChannel, sample: f32) -> f32 {
        let adjusted = self.mixer.apply_gain(channel, sample);
        for route in &self.routes {
            if self.policy.allows(route, channel) {
                route.forward(adjusted);
            }
        }
        adjusted
    }
}

fn main() {
    let mut server = AudioServer::new();
    server.register_route(AudioRoute::new("system", "master"));
    server.register_route(AudioRoute::new("media", "front-left"));

    let output = server.process_frame(AudioChannel::System, 0.75);
    println!("audio frame processed: {output}");
}
