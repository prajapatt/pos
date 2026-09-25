#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessEntry {
    pub pid: u32,
    pub name: String,
    pub status: String,
}

impl ProcessEntry {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            name: name.to_string(),
            status: "running".to_string(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProcessMonitor {
    entries: Vec<ProcessEntry>,
}

impl ProcessMonitor {
    pub fn register(&mut self, entry: ProcessEntry) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
