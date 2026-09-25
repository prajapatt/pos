use super::control_center::ControlCenterState;
use super::dock::Dock;
use super::launcher::Launcher;
use super::notifications::NotificationCenter;
use super::panel::PanelState;

pub struct DesktopState { pub launcher: Launcher, pub dock: Dock, pub notifications: NotificationCenter, pub panel: PanelState, pub control_center: ControlCenterState }
impl DesktopState { pub fn new() -> Self { Self { launcher: Launcher::default(), dock: Dock::default(), notifications: NotificationCenter::default(), panel: PanelState::default(), control_center: ControlCenterState::default() } } }
impl Default for DesktopState { fn default() -> Self { Self::new() } }
