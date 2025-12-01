use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::{BorrowedInterface, RawInterface};

#[repr(transparent)]
pub struct RendererGl2(pub(crate) BorrowedInterface<RenderInterfaceMarker>);

impl Into<RawInterface<RenderInterfaceMarker>> for &mut RendererGl2 {
    fn into(self) -> RawInterface<RenderInterfaceMarker> {
        RawInterface::new(self.0.raw)
    }
}
