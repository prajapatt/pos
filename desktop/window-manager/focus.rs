#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Focus {
    current: Option<u64>,
}

impl Focus {
    pub fn set(&mut self, id: u64) {
        self.current = Some(id);
    }

    pub fn clear(&mut self) {
        self.current = None;
    }

    pub fn current(&self) -> Option<u64> {
        self.current
    }

    pub fn is_focused(&self, id: u64) -> bool {
        self.current == Some(id)
    }
}

