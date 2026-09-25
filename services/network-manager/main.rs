mod connection;
mod dns;
mod firewall;
mod wifi;

use connection::{Connection, ConnectionStatus};
use dns::{DnsResolver, DnsRecord};
use firewall::{FirewallPolicy, FirewallRule};
use wifi::{WifiAccessPoint, WifiNetwork};

#[derive(Clone, Debug, Default)]
pub struct NetworkManager {
    connections: Vec<Connection>,
    dns: DnsResolver,
    firewall: FirewallPolicy,
    wifi: Option<WifiNetwork>,
}

impl NetworkManager {
    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn resolve(&mut self, name: &str) -> Option<DnsRecord> {
        self.dns.lookup(name)
    }

    pub fn apply_firewall(&mut self, rule: FirewallRule) {
        self.firewall.insert(rule);
    }

    pub fn connect_wifi(&mut self, wifi: WifiAccessPoint) {
        self.wifi = Some(WifiNetwork::new(wifi));
    }
}

fn main() {
    let mut manager = NetworkManager::default();
    manager.add_connection(Connection::new("eth0", "up", false));
    println!("connections: {}", manager.connections.len());
}
