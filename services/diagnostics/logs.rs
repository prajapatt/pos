#[derive(Clone, Debug, Default)]
pub struct LogCollector {
    entry_count: usize,
}

impl LogCollector {
    pub fn collect_summary(&self) -> String {
        format!("{} log entries collected", self.entry_count)
    }
}
