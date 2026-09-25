use core::cmp;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InodeKind {
    File,
    Directory,
    Device,
    Symlink,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Inode {
    pub id: u64,
    pub parent: Option<u64>,
    pub kind: InodeKind,
    pub size: u64,
    pub permissions: u16,
    pub links: u32,
}

impl Inode {
    pub const fn new(id: u64, parent: Option<u64>, kind: InodeKind, size: u64, permissions: u16) -> Self {
        Self {
            id,
            parent,
            kind,
            size,
            permissions,
            links: 1,
        }
    }

    pub fn is_readable(&self, uid: u32, gid: u32, requested: u16) -> bool {
        let owner_bits = (self.permissions >> 6) & 0b111;
        let group_bits = (self.permissions >> 3) & 0b111;
        let others_bits = self.permissions & 0b111;

        let effective = if uid == 0 {
            owner_bits
        } else if gid == 0 {
            group_bits
        } else {
            others_bits
        };

        (effective & requested) != 0
    }

    pub fn truncate(&mut self, len: u64) {
        self.size = cmp::min(self.size, len);
    }

    pub fn link(&mut self) {
        self.links = self.links.saturating_add(1);
    }

    pub fn unlink(&mut self) {
        self.links = self.links.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{Inode, InodeKind};

    #[test]
    fn inode_permission_checks_follow_basic_owner_rules() {
        let mut inode = Inode::new(10, None, InodeKind::File, 128, 0o644);
        assert!(inode.is_readable(0, 0, 0o4));
        assert!(!inode.is_readable(1, 1, 0o2));
        inode.permissions = 0o600;
        assert!(inode.is_readable(1, 1, 0o4));
    }
}
