mod process_monitor;
mod resource_policy;

use process_monitor::{ProcessEntry, ProcessMonitor};
use resource_policy::{ResourceLimit, ResourcePolicy};

#[derive(Clone, Debug, Default)]
pub struct ProcessManager {
    monitor: ProcessMonitor,
    policy: ResourcePolicy,
}

impl ProcessManager {
    pub fn add_process(&mut self, entry: ProcessEntry) {
        self.monitor.register(entry);
    }

    pub fn enforce_limit(&mut self, limit: ResourceLimit) {
        self.policy.apply(limit);
    }
}

fn main() {
    let mut manager = ProcessManager::default();
    manager.add_process(ProcessEntry::new(1, "init"));
    println!("processes: {}", manager.monitor.len());
}
