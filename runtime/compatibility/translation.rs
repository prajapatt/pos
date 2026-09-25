#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbiKind {
    Native,
    Linux,
    Win32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AbiTranslation {
    pub incoming: AbiKind,
    pub target: AbiKind,
    pub enabled: bool,
}

impl AbiTranslation {
    pub const fn new(incoming: AbiKind, target: AbiKind) -> Self {
        Self { incoming, target, enabled: true }
    }

    pub fn translate_syscall(&self, syscall_number: u32) -> u32 {
        if !self.enabled {
            return syscall_number;
        }

        match (self.incoming, self.target) {
            (AbiKind::Linux, AbiKind::Native) => syscall_number,
            (AbiKind::Win32, AbiKind::Native) => syscall_number + 0x1000,
            (AbiKind::Native, AbiKind::Linux) => syscall_number,
            (AbiKind::Native, AbiKind::Win32) => syscall_number.saturating_sub(0x1000),
            _ => syscall_number,
        }
    }

    pub fn can_translate(&self) -> bool {
        self.enabled && self.incoming != self.target
    }

    pub fn normalize(&self, syscall_number: u32) -> u32 {
        if !self.enabled {
            return syscall_number;
        }
        self.translate_syscall(syscall_number)
    }
}

impl Default for AbiTranslation {
    fn default() -> Self {
        Self::new(AbiKind::Native, AbiKind::Native)
    }
}

