#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PowerMode { Balanced, Performance, Saver }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PowerSettings { pub mode: PowerMode, pub idle_timeout_seconds: u32 }
impl Default for PowerSettings { fn default() -> Self { Self { mode: PowerMode::Balanced, idle_timeout_seconds: 900 } } }
