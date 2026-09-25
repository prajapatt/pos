#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecuritySettings { pub audit_enabled: bool, pub signed_packages_only: bool, pub ai_dangerous_approval: bool }
impl Default for SecuritySettings { fn default() -> Self { Self { audit_enabled: true, signed_packages_only: true, ai_dangerous_approval: true } } }
