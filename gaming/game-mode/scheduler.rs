#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulerClass { Normal, GameForeground }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchedulerPolicy { pub class: SchedulerClass, pub time_slice_ms: u32 }

impl SchedulerPolicy { pub const fn balanced() -> Self { Self { class: SchedulerClass::Normal, time_slice_ms: 10 } } pub const fn gaming() -> Self { Self { class: SchedulerClass::GameForeground, time_slice_ms: 4 } } }
