#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiMode { Local, Remote, Hybrid }
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AiSettings { pub mode: AiMode, pub verified_results_required: bool }
impl Default for AiSettings { fn default() -> Self { Self { mode: AiMode::Hybrid, verified_results_required: true } } }
