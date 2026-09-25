pub mod compositor {
    pub mod surfaces { include!("compositor/surfaces.rs"); }
    pub mod windows { include!("compositor/windows.rs"); }
    pub mod workspace { include!("compositor/workspace.rs"); }
    pub mod gpu { include!("compositor/gpu.rs"); }
    pub mod renderer { include!("compositor/renderer.rs"); }
    pub mod animations { include!("compositor/animations.rs"); }
    pub mod compositor { include!("compositor/compositor.rs"); }
}
pub mod window_manager {
    pub mod layout { include!("window-manager/layout.rs"); }
    pub mod focus { include!("window-manager/focus.rs"); }
    pub mod shortcuts { include!("window-manager/shortcuts.rs"); }
    pub mod workspaces { include!("window-manager/workspaces.rs"); }
    pub mod manager { include!("window-manager/manager.rs"); }
}
pub mod shell {
    pub mod launcher { include!("desktop-shell/launcher.rs"); }
    pub mod notifications { include!("desktop-shell/notifications.rs"); }
    pub mod panel { include!("desktop-shell/panel.rs"); }
    pub mod dock { include!("desktop-shell/dock.rs"); }
    pub mod control_center { include!("desktop-shell/control_center.rs"); }
    pub mod desktop { include!("desktop-shell/desktop.rs"); }
    pub mod shell { include!("desktop-shell/shell.rs"); }
}
pub mod settings {
    pub mod settings { include!("settings/settings.rs"); }
    pub mod display { include!("settings/display.rs"); }
    pub mod audio { include!("settings/audio.rs"); }
    pub mod network { include!("settings/network.rs"); }
    pub mod power { include!("settings/power.rs"); }
    pub mod privacy { include!("settings/privacy.rs"); }
    pub mod security { include!("settings/security.rs"); }
    pub mod ai { include!("settings/ai.rs"); }
}