use super::renderer::{RenderFrame, Renderer};
use super::surfaces::Rect;
use super::workspace::Workspace;

pub struct Compositor<R> { pub viewport: Rect, pub workspace: Workspace, renderer: R }
impl<R: Renderer> Compositor<R> {
	pub fn new(viewport: Rect, renderer: R) -> Self { Self { viewport, workspace: Workspace::default(), renderer } }
	pub fn render(&mut self) -> Result<(), String> { let ids = self.workspace.windows().iter().filter(|window| window.surface.visible).map(|window| window.id).collect(); self.renderer.render(&RenderFrame { viewport: self.viewport, surface_ids: ids }) }
}
