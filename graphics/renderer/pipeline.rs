use super::shader::Shader;

pub struct Pipeline { pub vertex: Shader, pub fragment: Shader }
impl Pipeline { pub fn new(vertex: Shader, fragment: Shader) -> Result<Self, &'static str> { if !matches!(vertex.stage, super::shader::ShaderStage::Vertex) || !matches!(fragment.stage, super::shader::ShaderStage::Fragment) { return Err("pipeline stages are incompatible"); } Ok(Self { vertex, fragment }) } }
