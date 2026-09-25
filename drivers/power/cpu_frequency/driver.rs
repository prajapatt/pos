#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrequencyPolicy { pub minimum_mhz: u32, pub maximum_mhz: u32, pub target_mhz: u32 }
impl FrequencyPolicy { pub fn validate(&self) -> Result<(), &'static str> { if self.minimum_mhz == 0 || self.minimum_mhz > self.maximum_mhz || self.target_mhz < self.minimum_mhz || self.target_mhz > self.maximum_mhz { Err("invalid CPU frequency policy") } else { Ok(()) } } }
