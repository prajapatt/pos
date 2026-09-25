pub const BOOT_PROTOCOL_MAGIC: u64 = 0x4348_5554_4f53_424f;
pub const BOOT_PROTOCOL_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LoadStage {
    Bios,
    EarlyKernel,
    Userspace,
    Runtime,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoaderConfig {
    pub stage: LoadStage,
    pub verbose: bool,
    pub timeout_ms: u32,
    pub load_address: u64,
    pub entry_point: u64,
}

impl LoaderConfig {
    pub const fn new(stage: LoadStage, load_address: u64, entry_point: u64) -> Self {
        Self {
            stage,
            verbose: false,
            timeout_ms: 5000,
            load_address,
            entry_point,
        }
    }

    pub fn set_verbose(&mut self, enabled: bool) {
        self.verbose = enabled;
    }

    pub fn is_valid(&self) -> bool {
        self.load_address != 0 && self.entry_point != 0 && self.timeout_ms > 0
    }
}

impl Default for LoaderConfig {
    fn default() -> Self {
        Self::new(LoadStage::EarlyKernel, 0x100000, 0x100000)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadedModule {
    pub base: u64,
    pub size: usize,
    pub entry: u64,
}

impl LoadedModule {
    pub const fn new(base: u64, size: usize, entry: u64) -> Self {
        Self { base, size, entry }
    }

    pub fn contains(&self, address: u64) -> bool {
        address >= self.base && address < self.base.saturating_add(self.size as u64)
    }

    pub fn is_valid(&self) -> bool {
        self.base != 0 && self.entry != 0 && self.size > 0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BootProtocol {
    pub magic: u64,
    pub version: u32,
    pub reserved: u32,
    pub kernel_base: u64,
    pub kernel_size: usize,
}

impl BootProtocol {
    pub const fn new(kernel_base: u64, kernel_size: usize) -> Self {
        Self {
            magic: BOOT_PROTOCOL_MAGIC,
            version: BOOT_PROTOCOL_VERSION,
            reserved: 0,
            kernel_base,
            kernel_size,
        }
    }

    pub fn validate(&self) -> bool {
        self.magic == BOOT_PROTOCOL_MAGIC
            && self.version == BOOT_PROTOCOL_VERSION
            && self.kernel_base != 0
            && self.kernel_size > 0
    }

    pub fn entry_point(&self) -> u64 {
        self.kernel_base
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoaderState {
    pub config: LoaderConfig,
    pub protocol: BootProtocol,
    pub loaded_module: LoadedModule,
}

impl LoaderState {
    pub const fn new(config: LoaderConfig, protocol: BootProtocol, loaded_module: LoadedModule) -> Self {
        Self { config, protocol, loaded_module }
    }

    pub fn is_ready(&self) -> bool {
        self.config.is_valid() && self.protocol.validate() && self.loaded_module.is_valid()
    }

    pub fn stage_transition(&mut self, next_stage: LoadStage) {
        self.config.stage = next_stage;
    }
}

#[cfg(test)]
mod tests {
    use super::{BootProtocol, LoaderConfig, LoadedModule, LoadStage, LoaderState};

    #[test]
    fn boot_protocol_rejects_invalid_metadata() {
        let invalid = BootProtocol::new(0, 0);
        assert!(!invalid.validate());
    }

    #[test]
    fn loader_state_is_ready_for_valid_kernel_payload() {
        let config = LoaderConfig::new(LoadStage::EarlyKernel, 0x100000, 0x100000);
        let protocol = BootProtocol::new(0x100000, 4096);
        let loaded = LoadedModule::new(0x100000, 4096, 0x100000);
        let state = LoaderState::new(config, protocol, loaded);

        assert!(config.is_valid());
        assert!(protocol.validate());
        assert!(loaded.is_valid());
        assert!(state.is_ready());
    }

    #[test]
    fn loaded_module_contains_address_range() {
        let module = LoadedModule::new(0x2000, 0x100, 0x2000);
        assert!(module.contains(0x2000));
        assert!(module.contains(0x20ff));
        assert!(!module.contains(0x2100));
    }
}
