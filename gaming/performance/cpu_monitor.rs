#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CpuSample { pub utilization_percent: f32, pub temperature_celsius: f32 }

impl CpuSample { pub fn is_safe(&self) -> bool { self.utilization_percent.is_finite() && self.temperature_celsius.is_finite() && (0.0..=100.0).contains(&self.utilization_percent) && self.temperature_celsius < 95.0 } }
