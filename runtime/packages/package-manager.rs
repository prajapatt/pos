#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageAction {
    Install,
    Update,
    Remove,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageRecord {
    pub name: &'static str,
    pub version: &'static str,
    pub action: PackageAction,
}

impl PackageRecord {
    pub const fn new(name: &'static str, version: &'static str, action: PackageAction) -> Self {
        Self { name, version, action }
    }
}
