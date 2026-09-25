use crate::fs::inode::{Inode, InodeKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectoryEntry {
    pub name: &'static str,
    pub inode_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Directory {
    pub inode: Inode,
    pub entries: Vec<DirectoryEntry>,
}

impl Directory {
    pub fn new(id: u64, parent: Option<u64>) -> Self {
        Self {
            inode: Inode::new(id, parent, InodeKind::Directory, 0, 0o755),
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, name: &'static str, inode_id: u64) -> Result<(), &'static str> {
        if name.is_empty() || name.contains('/') {
            return Err("invalid directory entry");
        }
        if self.entries.iter().any(|entry| entry.name == name) {
            return Err("entry exists");
        }
        self.entries.push(DirectoryEntry { name, inode_id });
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<u64> {
        self.entries.iter().find(|entry| entry.name == name).map(|entry| entry.inode_id)
    }

    pub fn remove(&mut self, name: &str) -> Option<DirectoryEntry> {
        let index = self.entries.iter().position(|entry| entry.name == name)?;
        Some(self.entries.remove(index))
    }
}
