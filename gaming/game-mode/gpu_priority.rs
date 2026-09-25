#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpuPriority { Normal, Foreground }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GpuPolicy { pub priority: GpuPriority, pub frame_pacing: bool }

impl GpuPolicy { pub const fn balanced() -> Self { Self { priority: GpuPriority::Normal, frame_pacing: false } } pub const fn gaming() -> Self { Self { priority: GpuPriority::Foreground, frame_pacing: true } } }
