pub mod game_mode {
    pub mod scheduler { include!("game-mode/scheduler.rs"); }
    pub mod memory { include!("game-mode/memory.rs"); }
    pub mod gpu_priority { include!("game-mode/gpu_priority.rs"); }
    pub mod background_services { include!("game-mode/background_services.rs"); }
    pub mod manager { include!("game-mode/manager.rs"); }
}
pub mod input {
    pub mod gamepad { include!("input/gamepad.rs"); }
    pub mod input_mapper { include!("input/input_mapper.rs"); }
}
pub mod performance {
    pub mod frame_monitor { include!("performance/frame_monitor.rs"); }
    pub mod cpu_monitor { include!("performance/cpu_monitor.rs"); }
    pub mod gpu_monitor { include!("performance/gpu_monitor.rs"); }
    pub mod latency { include!("performance/latency.rs"); }
}