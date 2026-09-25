#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SandboxPolicy {
    pub allow_network: bool,
    pub allow_disk: bool,
    pub allow_process_spawn: bool,
    pub max_threads: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Sandbox {
    pub policy: SandboxPolicy,
    pub active: bool,
}

impl Sandbox {
    pub const fn new(policy: SandboxPolicy) -> Self {
        Self { policy, active: true }
    }

    pub fn enforce(&self, request: &str) -> Result<(), &'static str> {
        match request {
            "network" if !self.policy.allow_network => Err("network access denied"),
            "disk" if !self.policy.allow_disk => Err("disk access denied"),
            "spawn" if !self.policy.allow_process_spawn => Err("process spawn denied"),
            _ => Ok(()),
        }
    }
}
