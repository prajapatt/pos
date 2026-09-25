mod boot_services;
mod service_manager;

use crate::boot_services::register_default_services;
use crate::service_manager::ServiceManager;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut manager = ServiceManager::new();
    register_default_services(&mut manager);

    if let Ok(config_path) = env::var("POS_BOOT_CONFIG") {
        if let Ok(configs) = crate::service_manager::load_boot_services(&config_path) {
            let configured_services = crate::boot_services::from_boot_config(&configs);
            let mut configured_manager = ServiceManager::new();
            for service in configured_services {
                configured_manager.register(service);
            }
            configured_manager.start_all();
            return ExitCode::SUCCESS;
        }
    }

    manager.start_all();
    println!("system init complete");
    ExitCode::SUCCESS
}
