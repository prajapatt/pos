#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Battery { pub design_capacity_mwh: u32, pub remaining_mwh: u32, pub charging: bool }
impl Battery { pub const fn percent(&self) -> u8 { if self.design_capacity_mwh == 0 { 0 } else { let value = self.remaining_mwh.saturating_mul(100) / self.design_capacity_mwh; if value > 100 { 100 } else { value as u8 } } } }
