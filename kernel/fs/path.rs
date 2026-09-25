pub const MAX_PATH_COMPONENTS: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathError { Empty, TooManyComponents, ParentTraversal, ComponentTooLong }

pub struct Path<'a> { components: [&'a str; MAX_PATH_COMPONENTS], count: usize, absolute: bool }
impl<'a> Path<'a> { pub fn parse(input: &'a str) -> Result<Self, PathError> { if input.is_empty() { return Err(PathError::Empty); } let absolute = input.as_bytes()[0] == b'/'; let mut path = Self { components: [""; MAX_PATH_COMPONENTS], count: 0, absolute }; for component in input.split('/') { if component.is_empty() || component == "." { continue; } if component == ".." { return Err(PathError::ParentTraversal); } if component.len() > 255 { return Err(PathError::ComponentTooLong); } if path.count == MAX_PATH_COMPONENTS { return Err(PathError::TooManyComponents); } path.components[path.count] = component; path.count += 1; } Ok(path) } pub const fn is_absolute(&self) -> bool { self.absolute } pub fn components(&self) -> &[&'a str] { &self.components[..self.count] } }
