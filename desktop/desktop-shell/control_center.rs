#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ControlCenterState { pub open: bool, pub network_online: bool, pub audio_muted: bool, pub power_policy: PowerPolicy }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerPolicy { Balanced, Performance, PowerSaver }
impl Default for ControlCenterState { fn default() -> Self { Self { open: false, network_online: false, audio_muted: false, power_policy: PowerPolicy::Balanced } } }
