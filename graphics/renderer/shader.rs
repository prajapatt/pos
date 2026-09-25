#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShaderStage { Vertex, Fragment }

pub struct Shader { pub stage: ShaderStage, bytecode: Vec<u32> }
impl Shader { pub fn new(stage: ShaderStage, bytecode: Vec<u32>) -> Result<Self, &'static str> { if bytecode.is_empty() { return Err("shader bytecode is empty"); } Ok(Self { stage, bytecode }) } pub fn words(&self) -> usize { self.bytecode.len() } }
