use rsmlui_sys::render_interface::RmlRenderInterface;

use crate::interfaces::{InterfaceHandle, InterfaceMarker, RawInterface};

pub struct RenderInterfaceMarker;

impl InterfaceMarker for RenderInterfaceMarker {
    type Ptr = *mut RmlRenderInterface;
}

impl<I> Into<RawInterface<RenderInterfaceMarker>> for &mut InterfaceHandle<I> {
    fn into(self) -> RawInterface<RenderInterfaceMarker> {
        todo!();
    }
}
