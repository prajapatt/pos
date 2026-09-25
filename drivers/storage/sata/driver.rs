#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
pub struct FisRegH2d { pub fis_type: u8, pub flags: u8, pub command: u8, pub feature_low: u8, pub lba: [u8; 6], pub device: u8, pub count: u16, pub reserved: [u8; 4] }

impl FisRegH2d { pub fn identify() -> Self { Self { fis_type: 0x27, flags: 0x80, command: 0xec, ..Self::default() } } }
