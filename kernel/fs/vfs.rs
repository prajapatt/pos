use crate::fs::inode::{Inode, InodeKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VfsError {
    NotFound,
    Exists,
    InvalidName,
    TooDeep,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VfsNode {
    pub name: &'static str,
    pub inode: Inode,
    pub children: Vec<VfsNode>,
}

impl VfsNode {
    pub fn new(name: &'static str, inode: Inode) -> Self {
        Self {
            name,
            inode,
            children: Vec::new(),
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&VfsNode> {
        self.children.iter().find(|child| child.name == name)
    }

    pub fn insert(&mut self, node: VfsNode) -> Result<(), VfsError> {
        if node.name.is_empty() || node.name.contains('/') {
            return Err(VfsError::InvalidName);
        }
        if self.lookup(node.name).is_some() {
            return Err(VfsError::Exists);
        }
        if self.children.len() >= 64 {
            return Err(VfsError::TooDeep);
        }
        self.children.push(node);
        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<VfsNode, VfsError> {
        let index = self
            .children
            .iter()
            .position(|child| child.name == name)
            .ok_or(VfsError::NotFound)?;
        Ok(self.children.remove(index))
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Vfs {
    root: Option<VfsNode>,
}

impl Vfs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mount_root(&mut self, root: VfsNode) {
        self.root = Some(root);
    }

    pub fn root(&self) -> Option<&VfsNode> {
        self.root.as_ref()
    }

    pub fn lookup(&self, path: &str) -> Result<&VfsNode, VfsError> {
        let root = self.root.as_ref().ok_or(VfsError::NotFound)?;
        if path == "/" || path.is_empty() {
            return Ok(root);
        }

        let mut current = root;
        for segment in path.split('/').filter(|part| !part.is_empty()) {
            current = current.lookup(segment).ok_or(VfsError::NotFound)?;
        }
        Ok(current)
    }

    pub fn create_file(&mut self, path: &str, inode_id: u64) -> Result<(), VfsError> {
        let parent = self.parent_of(path)?;
        let name = self.name_of(path)?;
        let node = VfsNode::new(name, Inode::new(inode_id, None, InodeKind::File, 0, 0o644));

        if let Some(parent_node) = self.root.as_mut() {
            let mut current = parent_node;
            for segment in self.path_segments(path).iter().take(self.path_segments(path).len().saturating_sub(1)) {
                current = current.lookup(segment).ok_or(VfsError::NotFound)?;
            }
            current.insert(node)
        } else {
            Err(VfsError::NotFound)
        }
    }

    fn parent_of(&self, path: &str) -> Result<&VfsNode, VfsError> {
        let root = self.root.as_ref().ok_or(VfsError::NotFound)?;
        let mut current = root;
        let mut segments = path.split('/').filter(|part| !part.is_empty()).collect::<Vec<_>>();
        if segments.is_empty() {
            return Ok(root);
        }
        segments.pop();
        for segment in segments {
            current = current.lookup(segment).ok_or(VfsError::NotFound)?;
        }
        Ok(current)
    }

    fn name_of(&self, path: &str) -> Result<&'static str, VfsError> {
        let name = path.rsplit('/').next().filter(|segment| !segment.is_empty()).ok_or(VfsError::InvalidName)?;
        Ok(Box::leak(name.to_string().into_boxed_str()))
    }

    fn path_segments<'a>(&'a self, path: &str) -> Vec<&'a str> {
        path.split('/').filter(|part| !part.is_empty()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Vfs, VfsNode};
    use crate::fs::inode::{Inode, InodeKind};

    #[test]
    fn vfs_can_mount_and_lookup_root_entries() {
        let mut vfs = Vfs::new();
        let root = VfsNode::new("/", Inode::new(1, None, InodeKind::Directory, 0, 0o755));
        vfs.mount_root(root);

        let file = VfsNode::new("etc", Inode::new(2, Some(1), InodeKind::Directory, 0, 0o755));
        vfs.root.as_mut().unwrap().insert(file).unwrap();
        assert!(vfs.lookup("/").is_ok());
    }
}
