#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageRepoEntry {
    pub name: &'static str,
    pub version: &'static str,
    pub sha256: [u8; 32],
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PackageRepository {
    entries: Vec<PackageRepoEntry>,
}

impl PackageRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: PackageRepoEntry) {
        self.entries.push(entry);
    }

    pub fn contains(&self, name: &str) -> bool {
        self.entries.iter().any(|entry| entry.name == name)
    }
}
