#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PanelState { pub visible: bool, pub workspace: usize, pub unread_notifications: usize }
impl Default for PanelState { fn default() -> Self { Self { visible: true, workspace: 0, unread_notifications: 0 } } }
