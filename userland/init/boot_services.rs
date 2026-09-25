use crate::service_manager::{BootServiceConfig, ServiceDefinition, ServiceManager};
use std::time::Duration;

pub fn default_boot_services() -> Vec<ServiceDefinition> {
    vec![
        ServiceDefinition {
            name: "kernel-logger".to_string(),
            command: vec!["/usr/sbin/klogd".to_string()],
            depends_on: vec![],
            restart_on_failure: true,
            restart_delay: Duration::from_secs(1),
            max_restarts: 3,
            working_dir: None,
        },
        ServiceDefinition {
            name: "device-manager".to_string(),
            command: vec!["/usr/sbin/device-manager".to_string()],
            depends_on: vec!["kernel-logger".to_string()],
            restart_on_failure: true,
            restart_delay: Duration::from_secs(2),
            max_restarts: 5,
            working_dir: None,
        },
        ServiceDefinition {
            name: "network-manager".to_string(),
            command: vec!["/usr/sbin/network-manager".to_string()],
            depends_on: vec!["device-manager".to_string()],
            restart_on_failure: true,
            restart_delay: Duration::from_secs(2),
            max_restarts: 5,
            working_dir: None,
        },
    ]
}

pub fn register_default_services(manager: &mut ServiceManager) {
    for service in default_boot_services() {
        manager.register(service);
    }
}

pub fn from_boot_config(configs: &[BootServiceConfig]) -> Vec<ServiceDefinition> {
    configs
        .iter()
        .filter(|cfg| cfg.enabled)
        .map(|cfg| ServiceDefinition {
            name: cfg.name.clone(),
            command: cfg.command.clone(),
            depends_on: cfg.depends_on.clone(),
            restart_on_failure: true,
            restart_delay: Duration::from_secs(1),
            max_restarts: 5,
            working_dir: None,
        })
        .collect()
}
