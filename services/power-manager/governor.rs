#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GovernorMode {
    Performance,
    Balanced,
    Powersave,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PowerGovernor {
    mode: GovernorMode,
}

impl PowerGovernor {
    pub fn set_mode(&mut self, mode: GovernorMode) {
        self.mode = mode;
    }

    pub fn current(&self) -> GovernorMode {
        self.mode
    }
}
