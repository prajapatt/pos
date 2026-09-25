#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BinaryFormat {
    Elf,
    Pe,
    MachO,
    Script,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BinaryMetadata {
    pub format: BinaryFormat,
    pub entry_point: u64,
    pub image_size: usize,
}

impl BinaryMetadata {
    pub const fn new(format: BinaryFormat, entry_point: u64, image_size: usize) -> Self {
        Self { format, entry_point, image_size }
    }

    pub fn is_supported(&self) -> bool {
        matches!(self.format, BinaryFormat::Elf | BinaryFormat::Pe)
    }

    pub fn is_valid(&self) -> bool {
        self.image_size > 0 && self.entry_point != 0
    }

    pub fn page_size(&self) -> usize {
        4096
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LoadRequest {
    pub format: BinaryFormat,
    pub preferred_address: u64,
    pub entry_point: u64,
    pub image_size: usize,
}

impl LoadRequest {
    pub const fn new(format: BinaryFormat, preferred_address: u64, entry_point: u64, image_size: usize) -> Self {
        Self { format, preferred_address, entry_point, image_size }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BinaryLoader {
    pub base_address: u64,
    pub loaded: usize,
}

impl BinaryLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&mut self, request: LoadRequest) -> Result<BinaryMetadata, &'static str> {
        let metadata = BinaryMetadata::new(request.format, request.entry_point, request.image_size);
        if !metadata.is_valid() {
            return Err("invalid binary metadata");
        }
        if !metadata.is_supported() {
            return Err("unsupported binary format");
        }
        self.base_address = request.preferred_address;
        self.loaded = metadata.image_size;
        Ok(metadata)
    }

    pub fn load_address(&self) -> u64 {
        self.base_address
    }
}

