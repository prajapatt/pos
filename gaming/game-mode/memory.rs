#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryPolicy { pub working_set_reserve_bytes: u64, pub reclaim_background_cache: bool }

impl MemoryPolicy { pub const fn balanced() -> Self { Self { working_set_reserve_bytes: 0, reclaim_background_cache: false } } pub const fn gaming(reserve_bytes: u64) -> Self { Self { working_set_reserve_bytes: reserve_bytes, reclaim_background_cache: true } } }
