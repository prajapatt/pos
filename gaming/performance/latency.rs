#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LatencyStats { pub samples: u64, pub average_ms: f32, pub worst_ms: f32 }

impl LatencyStats { pub fn record(&mut self, latency_ms: f32) { if !latency_ms.is_finite() || latency_ms < 0.0 { return; } self.samples += 1; self.average_ms += (latency_ms - self.average_ms) / self.samples as f32; self.worst_ms = self.worst_ms.max(latency_ms); } }
