#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GpuSample { pub utilization_percent: f32, pub memory_used_bytes: u64, pub frame_time_ms: f32 }

impl GpuSample { pub fn valid(&self) -> bool { self.utilization_percent.is_finite() && (0.0..=100.0).contains(&self.utilization_percent) && self.frame_time_ms.is_finite() && self.frame_time_ms >= 0.0 } }
