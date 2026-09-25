#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Layout {
    Tiled,
    Stacked,
    Floating,
}

impl Layout {
    pub fn next(self) -> Self {
        match self {
            Self::Tiled => Self::Stacked,
            Self::Stacked => Self::Floating,
            Self::Floating => Self::Tiled,
        }
    }

    pub fn cycle(&mut self) {
        *self = self.next();
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self::Tiled
    }
}

