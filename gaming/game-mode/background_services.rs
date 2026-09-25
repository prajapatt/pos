#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackgroundPolicy { pub indexing_enabled: bool, pub update_downloads_enabled: bool, pub telemetry_enabled: bool }

impl BackgroundPolicy { pub const fn normal() -> Self { Self { indexing_enabled: true, update_downloads_enabled: true, telemetry_enabled: true } } pub const fn gaming() -> Self { Self { indexing_enabled: false, update_downloads_enabled: false, telemetry_enabled: true } } }
