#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionStatus {
    Up,
    Down,
    Degraded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Connection {
    pub interface: String,
    pub status: ConnectionStatus,
    pub is_default: bool,
}

impl Connection {
    pub fn new(interface: &str, status: &str, is_default: bool) -> Self {
        let status = match status {
            "up" => ConnectionStatus::Up,
            "degraded" => ConnectionStatus::Degraded,
            _ => ConnectionStatus::Down,
        };
        Self {
            interface: interface.to_string(),
            status,
            is_default,
        }
    }
}
