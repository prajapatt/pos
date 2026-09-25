use std::collections::HashMap;
use std::env;
use std::process::ExitCode;

#[derive(Clone, Debug, Default)]
pub struct NetworkInterface {
    name: String,
    state: String,
    ip: String,
}

#[derive(Clone, Debug, Default)]
pub struct NetworkManager {
    interfaces: HashMap<String, NetworkInterface>,
}

impl NetworkManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.interfaces.insert(
            "lo".to_string(),
            NetworkInterface {
                name: "lo".to_string(),
                state: "up".to_string(),
                ip: "127.0.0.1/8".to_string(),
            },
        );
        manager
    }

    pub fn up(&mut self, name: &str) {
        let entry = self.interfaces.entry(name.to_string()).or_insert_with(|| NetworkInterface {
            name: name.to_string(),
            state: "down".to_string(),
            ip: "0.0.0.0/0".to_string(),
        });
        entry.state = "up".to_string();
    }

    pub fn down(&mut self, name: &str) {
        if let Some(entry) = self.interfaces.get_mut(name) {
            entry.state = "down".to_string();
        }
    }

    pub fn set_ip(&mut self, name: &str, ip: &str) {
        if let Some(entry) = self.interfaces.get_mut(name) {
            entry.ip = ip.to_string();
        }
    }

    pub fn list(&self) {
        for iface in self.interfaces.values() {
            println!("{} {} {}", iface.name, iface.state, iface.ip);
        }
    }
}

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let mut manager = NetworkManager::new();

    match args.next().as_deref() {
        Some("up") => {
            let iface = args.next().unwrap_or_else(|| "lo".to_string());
            manager.up(&iface);
            println!("interface {iface} started");
            ExitCode::SUCCESS
        }
        Some("down") => {
            let iface = args.next().unwrap_or_else(|| "lo".to_string());
            manager.down(&iface);
            println!("interface {iface} stopped");
            ExitCode::SUCCESS
        }
        Some("set-ip") => {
            let iface = args.next().unwrap_or_else(|| "lo".to_string());
            let ip = args.next().unwrap_or_else(|| "127.0.0.1/8".to_string());
            manager.set_ip(&iface, &ip);
            println!("{iface} -> {ip}");
            ExitCode::SUCCESS
        }
        Some("list") | Some("status") => {
            manager.list();
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("usage: netctl [up|down|set-ip|list|status]");
            ExitCode::from(2)
        }
    }
}
