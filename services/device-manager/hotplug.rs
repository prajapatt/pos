#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DeviceEventKind {
    Added,
    Removed,
    Updated,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceEvent {
    pub device: String,
    pub kind: DeviceEventKind,
    pub details: String,
}

impl DeviceEvent {
    pub fn new(device: &str, kind: DeviceEventKind, details: &str) -> Self {
        Self {
            device: device.to_string(),
            kind,
            details: details.to_string(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct HotplugObserver {
    last_seen: String,
}

impl HotplugObserver {
    pub fn notify(&mut self, event: DeviceEvent) {
        self.last_seen = event.device.clone();
        let _ = event.kind;
    }
}
