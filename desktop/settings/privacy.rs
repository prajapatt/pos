#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrivacySettings { pub telemetry: bool, pub screen_capture_requires_approval: bool }
impl Default for PrivacySettings { fn default() -> Self { Self { telemetry: false, screen_capture_requires_approval: true } } }
