#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FrameStats { pub samples: u64, pub average_ms: f32, pub worst_ms: f32 }

#[derive(Default)]
pub struct FrameMonitor { stats: FrameStats }
impl FrameMonitor { pub fn record(&mut self, frame_ms: f32) { if !frame_ms.is_finite() || frame_ms < 0.0 { return; } self.stats.samples += 1; self.stats.average_ms += (frame_ms - self.stats.average_ms) / self.stats.samples as f32; self.stats.worst_ms = self.stats.worst_ms.max(frame_ms); } pub const fn stats(&self) -> FrameStats { self.stats } }
