use super::gpu::GpuBackend;
use super::surfaces::Rect;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderFrame { pub viewport: Rect, pub surface_ids: Vec<u64> }

pub trait Renderer { fn render(&mut self, frame: &RenderFrame) -> Result<(), String>; }

pub struct BackendRenderer { backend: GpuBackend, last_frame: Option<RenderFrame> }
impl BackendRenderer { pub fn new(backend: GpuBackend) -> Self { Self { backend, last_frame: None } } pub fn backend(&self) -> GpuBackend { self.backend } }
impl Renderer for BackendRenderer { fn render(&mut self, frame: &RenderFrame) -> Result<(), String> { if frame.viewport.width == 0 || frame.viewport.height == 0 { return Err("invalid viewport".into()); } self.last_frame = Some(frame.clone()); Ok(()) } }
