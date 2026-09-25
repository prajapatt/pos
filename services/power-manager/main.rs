mod governor;
mod sleep;

use governor::{PowerGovernor, GovernorMode};
use sleep::{SleepPolicy, SleepState};

#[derive(Clone, Debug, Default)]
pub struct PowerManager {
    governor: PowerGovernor,
    sleep_policy: SleepPolicy,
}

impl PowerManager {
    pub fn update_mode(&mut self, mode: GovernorMode) {
        self.governor.set_mode(mode);
    }

    pub fn request_sleep(&mut self, state: SleepState) -> bool {
        self.sleep_policy.can_enter(state)
    }
}

fn main() {
    let mut manager = PowerManager::default();
    manager.update_mode(GovernorMode::Balanced);
    println!("sleep allowed: {}", manager.request_sleep(SleepState::Suspend));
}
