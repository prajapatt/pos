#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    OpenLauncher,
    CloseWindow,
    NextWindow,
    ToggleOverview,
}

#[derive(Default)]
pub struct ShortcutMap {
    bindings: Vec<(String, Action)>,
}

impl ShortcutMap {
    pub fn bind(&mut self, key: impl Into<String>, action: Action) {
        let key = key.into();
        if !self.bindings.iter().any(|(binding, _)| binding == &key) {
            self.bindings.push((key, action));
        }
    }

    pub fn unbind(&mut self, key: &str) {
        self.bindings.retain(|(binding, _)| binding != key);
    }

    pub fn action(&self, key: &str) -> Option<Action> {
        self.bindings
            .iter()
            .find(|(binding, _)| binding == key)
            .map(|(_, action)| *action)
    }

    pub fn bindings(&self) -> &[(String, Action)] {
        &self.bindings
    }
}

