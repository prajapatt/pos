#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeState {
    Stopped,
    Starting,
    Running,
    Crashed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppRuntime {
    pub name: &'static str,
    pub state: RuntimeState,
    pub pid: u32,
    pub memory_limit: usize,
}

impl AppRuntime {
    pub fn new(name: &'static str, pid: u32, memory_limit: usize) -> Self {
        Self {
            name,
            state: RuntimeState::Stopped,
            pid,
            memory_limit,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.memory_limit == 0 {
            return Err("runtime memory limit must be non-zero");
        }
        self.state = RuntimeState::Starting;
        self.state = RuntimeState::Running;
        Ok(())
    }

    pub fn stop(&mut self) {
        self.state = RuntimeState::Stopped;
    }

    pub fn crash(&mut self) {
        self.state = RuntimeState::Crashed;
    }
}
