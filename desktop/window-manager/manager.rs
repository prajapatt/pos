use super::focus::Focus;
use super::layout::Layout;
use super::shortcuts::{Action, ShortcutMap};
use super::workspaces::WorkspaceSet;

pub struct WindowManager {
    pub layout: Layout,
    pub focus: Focus,
    pub workspaces: WorkspaceSet,
    pub shortcuts: ShortcutMap,
}

impl WindowManager {
    pub fn new() -> Self {
        let mut shortcuts = ShortcutMap::default();
        shortcuts.bind("Super", Action::OpenLauncher);
        shortcuts.bind("Alt+Tab", Action::NextWindow);
        shortcuts.bind("Super+Shift+Q", Action::CloseWindow);

        Self {
            layout: Layout::Tiled,
            focus: Focus::default(),
            workspaces: WorkspaceSet::new(4),
            shortcuts,
        }
    }

    pub fn activate_workspace(&mut self, index: usize) -> bool {
        self.workspaces.switch(index)
    }

    pub fn focus_window(&mut self, id: u64) {
        self.focus.set(id);
    }

    pub fn open_window(&mut self, id: u64) {
        self.workspaces.add_window(id);
        self.focus_window(id);
    }

    pub fn close_window(&mut self, id: u64) {
        self.workspaces.remove_window(id);
        if self.focus.current() == Some(id) {
            self.focus.clear();
        }
    }

    pub fn cycle_layout(&mut self) {
        self.layout.cycle();
    }

    pub fn resolve_action(&self, key: &str) -> Option<Action> {
        self.shortcuts.action(key)
    }
}

impl Default for WindowManager {
    fn default() -> Self {
        Self::new()
    }
}

