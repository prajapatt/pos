#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WifiAccessPoint {
    pub ssid: String,
    pub bssid: String,
    pub channel: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WifiNetwork {
    pub ap: WifiAccessPoint,
    pub signal: u8,
    pub connected: bool,
}

impl WifiNetwork {
    pub fn new(ap: WifiAccessPoint) -> Self {
        Self {
            ap,
            signal: 80,
            connected: true,
        }
    }
}
