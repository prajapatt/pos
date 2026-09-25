#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WifiState { Disabled, Scanning, Associated, Failed }

pub struct WifiDevice { state: WifiState, channel: Option<u8> }
impl WifiDevice { pub const fn new() -> Self { Self { state: WifiState::Disabled, channel: None } } pub fn start_scan(&mut self) { self.state = WifiState::Scanning; self.channel = None; } pub fn associate(&mut self, channel: u8) -> Result<(), &'static str> { if !(1..=196).contains(&channel) { self.state = WifiState::Failed; return Err("invalid Wi-Fi channel"); } self.channel = Some(channel); self.state = WifiState::Associated; Ok(()) } pub const fn state(&self) -> WifiState { self.state } }
