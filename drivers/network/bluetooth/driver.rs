#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HciCommand { pub opcode: u16, pub parameters_length: u8 }

impl HciCommand { pub const fn reset() -> Self { Self { opcode: 0x0c03, parameters_length: 0 } } pub const fn le_set_scan_enable(enable: bool) -> Self { Self { opcode: 0x200c, parameters_length: if enable { 2 } else { 2 } } } }
