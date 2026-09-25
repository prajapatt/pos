#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpdateStage {
    Downloaded,
    Verified,
    Installed,
    Active,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpdateTarget {
    pub component: String,
    pub version: String,
    pub stage: UpdateStage,
}

impl UpdateTarget {
    pub fn new(component: &str, version: &str) -> Self {
        Self {
            component: component.to_string(),
            version: version.to_string(),
            stage: UpdateStage::Downloaded,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct UpdateQueue {
    items: Vec<UpdateTarget>,
}

impl UpdateQueue {
    pub fn enqueue(&mut self, target: UpdateTarget) {
        self.items.push(target);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
