pub const HDA_GCTL: usize = 0x08;
pub const HDA_STATE_STATUS: usize = 0x0e;
pub const HDA_CORB_BASE: usize = 0x40;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CodecVerb(pub u32);

impl CodecVerb { pub const fn get_parameter(codec: u8, node: u8, parameter: u16) -> Self { Self(((codec as u32) << 28) | ((node as u32) << 20) | (0xf00 << 8) | parameter as u32) } }
