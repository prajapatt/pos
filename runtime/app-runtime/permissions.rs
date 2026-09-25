#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppPermission {
    pub id: &'static str,
    pub allow: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermissionSet {
    pub permissions: Vec<AppPermission>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self { permissions: Vec::new() }
    }

    pub fn add(&mut self, permission: AppPermission) {
        self.permissions.push(permission);
    }

    pub fn has(&self, id: &str) -> bool {
        self.permissions.iter().any(|perm| perm.id == id && perm.allow)
    }
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self::new()
    }
}
