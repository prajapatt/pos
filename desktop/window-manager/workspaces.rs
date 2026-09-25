#[derive(Clone, Debug, Default)]
pub struct WorkspaceSet {
    count: usize,
    active: usize,
    windows: Vec<Vec<u64>>,
}

impl WorkspaceSet {
    pub fn new(count: usize) -> Self {
        let count = count.max(1);
        Self {
            count,
            active: 0,
            windows: vec![Vec::new(); count],
        }
    }

    pub fn switch(&mut self, index: usize) -> bool {
        if index >= self.count {
            return false;
        }
        self.active = index;
        true
    }

    pub fn active(&self) -> usize {
        self.active
    }

    pub fn add_window(&mut self, window_id: u64) {
        self.windows[self.active].push(window_id);
    }

    pub fn remove_window(&mut self, window_id: u64) {
        for workspace in &mut self.windows {
            workspace.retain(|id| *id != window_id);
        }
    }

    pub fn windows_in_active_workspace(&self) -> &[u64] {
        &self.windows[self.active]
    }
}

