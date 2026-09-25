use crate::fs::inode::{Inode, InodeKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MountError {
    AlreadyMounted,
    MissingSource,
    MissingTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MountPoint {
    pub source: &'static str,
    pub target: &'static str,
    pub fs_type: &'static str,
    pub inode: Inode,
}

impl MountPoint {
    pub const fn new(source: &'static str, target: &'static str, fs_type: &'static str, inode_id: u64) -> Self {
        Self {
            source,
            target,
            fs_type,
            inode: Inode::new(inode_id, None, InodeKind::Directory, 0, 0o755),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MountTable {
    mounts: Vec<MountPoint>,
}

impl MountTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mount(&mut self, point: MountPoint) -> Result<(), MountError> {
        if point.source.is_empty() || point.target.is_empty() {
            return Err(MountError::MissingSource);
        }
        if self.mounts.iter().any(|m| m.target == point.target) {
            return Err(MountError::AlreadyMounted);
        }
        self.mounts.push(point);
        Ok(())
    }

    pub fn unmount(&mut self, target: &str) -> Result<MountPoint, MountError> {
        let index = self
            .mounts
            .iter()
            .position(|m| m.target == target)
            .ok_or(MountError::MissingTarget)?;
        Ok(self.mounts.remove(index))
    }

    pub fn contains(&self, target: &str) -> bool {
        self.mounts.iter().any(|m| m.target == target)
    }
}
