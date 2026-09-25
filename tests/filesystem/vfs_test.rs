extern crate pos_kernel;

use pos_kernel::fs::inode::{Inode, InodeKind};
use pos_kernel::fs::vfs::{Vfs, VfsNode};

#[test]
fn vfs_mounts_root_and_looks_up_top_level_entries() {
    let mut vfs = Vfs::new();
    let root = VfsNode::new("/", Inode::new(1, None, InodeKind::Directory, 0, 0o755));
    vfs.mount_root(root);

    let etc = VfsNode::new("etc", Inode::new(2, Some(1), InodeKind::Directory, 0, 0o755));
    vfs.root().unwrap().insert(etc).unwrap();

    assert_eq!(vfs.lookup("/").unwrap().name, "/");
    assert_eq!(vfs.lookup("/etc").unwrap().name, "etc");
}

#[test]
fn vfs_lookup_tracks_nested_paths() {
    let mut vfs = Vfs::new();
    let root = VfsNode::new("/", Inode::new(1, None, InodeKind::Directory, 0, 0o755));
    vfs.mount_root(root);

    let etc = VfsNode::new("etc", Inode::new(2, Some(1), InodeKind::Directory, 0, 0o755));
    let config = VfsNode::new("config", Inode::new(3, Some(2), InodeKind::Directory, 0, 0o755));

    vfs.root().unwrap().insert(etc).unwrap();
    vfs.root().unwrap().lookup("etc").unwrap().insert(config).unwrap();

    assert_eq!(vfs.lookup("/etc/config").unwrap().name, "config");
}

#[test]
fn vfs_can_create_files_in_existing_directories() {
    let mut vfs = Vfs::new();
    let root = VfsNode::new("/", Inode::new(1, None, InodeKind::Directory, 0, 0o755));
    vfs.mount_root(root);

    let etc = VfsNode::new("etc", Inode::new(2, Some(1), InodeKind::Directory, 0, 0o755));
    vfs.root().unwrap().insert(etc).unwrap();

    vfs.create_file("/etc/hosts", 42).unwrap();

    let hosts = vfs.lookup("/etc/hosts").unwrap();
    assert_eq!(hosts.name, "hosts");
    assert_eq!(hosts.inode.id, 42);
    assert_eq!(hosts.inode.kind, InodeKind::File);
}
