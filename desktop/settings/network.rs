#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkSettings { pub interface: String, pub dhcp: bool, pub enabled: bool }
impl Default for NetworkSettings { fn default() -> Self { Self { interface: "auto".into(), dhcp: true, enabled: true } } }
