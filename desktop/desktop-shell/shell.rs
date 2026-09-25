use super::desktop::DesktopState;

pub struct DesktopShell { pub state: DesktopState, running: bool }
impl DesktopShell { pub fn new() -> Self { Self { state: DesktopState::new(), running: false } } pub fn start(&mut self) { self.running = true; } pub fn stop(&mut self) { self.running = false; } pub fn is_running(&self) -> bool { self.running } }
impl Default for DesktopShell { fn default() -> Self { Self::new() } }
