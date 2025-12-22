use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::{BorrowedInterface, RawInterface};
use crate::not_send_sync;

#[repr(transparent)]
pub struct RendererGl2(pub(crate) BorrowedInterface<RenderInterfaceMarker>);

not_send_sync!(RendererGl2);

impl Into<RawInterface<RenderInterfaceMarker>> for &mut RendererGl2 {
    fn into(self) -> RawInterface<RenderInterfaceMarker> {
        RawInterface::new(self.0.raw)
    }
}
