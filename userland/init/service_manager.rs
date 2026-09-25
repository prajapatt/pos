use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
    Stopping,
}

#[derive(Debug, Clone)]
pub struct ServiceDefinition {
    pub name: String,
    pub command: Vec<String>,
    pub depends_on: Vec<String>,
    pub restart_on_failure: bool,
    pub restart_delay: Duration,
    pub max_restarts: usize,
    pub working_dir: Option<String>,
}

#[derive(Debug)]
pub struct ServiceInstance {
    pub definition: ServiceDefinition,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub restarts: usize,
    pub started_at: Option<Instant>,
    pub last_exit_status: Option<ExitStatus>,
    pub child: Option<Child>,
}

#[derive(Debug, Default)]
pub struct ServiceManager {
    services: HashMap<String, ServiceInstance>,
    ordering: Vec<String>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, definition: ServiceDefinition) {
        let name = definition.name.clone();
        self.services.insert(
            name.clone(),
            ServiceInstance {
                definition: definition.clone(),
                state: ServiceState::Stopped,
                pid: None,
                restarts: 0,
                started_at: None,
                last_exit_status: None,
                child: None,
            },
        );
        self.ordering.push(name);
    }

    fn dependency_chain(&self, name: &str, visited: &mut HashSet<String>) -> Vec<String> {
        let mut order = Vec::new();
        if visited.insert(name.to_string()) {
            if let Some(service) = self.services.get(name) {
                for dependency in &service.definition.depends_on {
                    order.extend(self.dependency_chain(dependency, visited));
                }
                order.push(name.to_string());
            }
        }
        order
    }

    pub fn start_all(&mut self) {
        let mut names = Vec::new();
        for service_name in self.ordering.iter().cloned() {
            let mut visited = HashSet::new();
            names.extend(self.dependency_chain(&service_name, &mut visited));
        }

        let mut seen = HashSet::new();
        for name in names {
            if seen.insert(name.clone()) {
                self.start_service(&name);
            }
        }
    }

    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name).ok_or_else(|| format!("unknown service: {name}"))?;

        for dependency in &service.definition.depends_on {
            if let Some(dep) = self.services.get_mut(dependency) {
                if dep.state != ServiceState::Running {
                    self.start_service(dependency)?;
                }
            } else {
                return Err(format!("missing dependency: {dependency}"));
            }
        }

        if service.state == ServiceState::Running {
            return Ok(());
        }

        let mut command = Command::new(&service.definition.command[0]);
        command.args(&service.definition.command[1..]);

        if let Some(dir) = &service.definition.working_dir {
            command.current_dir(dir);
        }

        let child = command.spawn().map_err(|err| format!("failed to start service {name}: {err}"))?;
        let pid = child.id();

        service.pid = Some(pid);
        service.state = ServiceState::Starting;
        service.started_at = Some(Instant::now());
        service.child = Some(child);
        Ok(())
    }

    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        let service = self.services.get_mut(name).ok_or_else(|| format!("unknown service: {name}"))?;
        service.state = ServiceState::Stopping;

        if let Some(child) = service.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }

        service.pid = None;
        service.child = None;
        service.state = ServiceState::Stopped;
        Ok(())
    }

    pub fn poll(&mut self) {
        let mut to_restart = Vec::new();

        for (name, service) in self.services.iter_mut() {
            if service.child.is_none() {
                continue;
            }

            let child = service.child.as_mut().unwrap();
            match child.try_wait() {
                Ok(Some(status)) => {
                    service.last_exit_status = Some(status);
                    service.pid = None;
                    service.child = None;
                    service.state = if service.definition.restart_on_failure && service.restarts < service.definition.max_restarts {
                        to_restart.push(name.clone());
                        ServiceState::Failed
                    } else {
                        ServiceState::Stopped
                    };
                }
                Ok(None) => {
                    service.state = ServiceState::Running;
                }
                Err(_) => {
                    service.state = ServiceState::Failed;
                }
            }
        }

        for name in to_restart {
            if let Some(service) = self.services.get_mut(&name) {
                service.restarts += 1;
                std::thread::sleep(service.definition.restart_delay);
                let _ = self.start_service(&name);
            }
        }
    }

    pub fn list_services(&self) -> Vec<(&String, &ServiceInstance)> {
        self.services.iter().collect()
    }
}

#[derive(Debug, Clone)]
pub struct BootServiceConfig {
    pub name: String,
    pub command: Vec<String>,
    pub depends_on: Vec<String>,
    pub enabled: bool,
}

pub fn load_boot_services(config_path: &str) -> Result<Vec<BootServiceConfig>, String> {
    let content = fs::read_to_string(config_path)
        .map_err(|err| format!("unable to read boot service config {config_path}: {err}"))?;

    let mut services = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        let name = parts[0].to_string();
        let command = parts[1..].to_vec().into_iter().map(str::to_string).collect();
        services.push(BootServiceConfig {
            name,
            command,
            depends_on: Vec::new(),
            enabled: true,
        });
    }
    Ok(services)
}
