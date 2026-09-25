#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelDescriptor {
	pub name: String,
	pub provider: String,
	pub local: bool,
}

#[derive(Default)]
pub struct ModelRegistry {
	models: Vec<ModelDescriptor>,
}

impl ModelRegistry {
	pub fn register(&mut self, descriptor: ModelDescriptor) -> bool {
		if self.models.iter().any(|model| model.name == descriptor.name) {
			return false;
		}
		self.models.push(descriptor);
		true
	}

	pub fn find(&self, name: &str) -> Option<&ModelDescriptor> {
		self.models.iter().find(|model| model.name == name)
	}

	pub fn models(&self) -> &[ModelDescriptor] {
		&self.models
	}
}
