#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Exception { DivideByZero, Debug, PageFault, GeneralProtection, DoubleFault, Unknown(u8) }
impl From<u8> for Exception { fn from(vector: u8) -> Self { match vector { 0 => Self::DivideByZero, 1 => Self::Debug, 8 => Self::DoubleFault, 13 => Self::GeneralProtection, 14 => Self::PageFault, value => Self::Unknown(value) } } }
