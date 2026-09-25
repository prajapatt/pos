pub const IRQ_BASE: u8 = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Irq { pub vector: u8, pub line: u8 }
impl Irq { pub const fn new(line: u8) -> Option<Self> { if line < 16 { Some(Self { vector: IRQ_BASE + line, line }) } else { None } } }
