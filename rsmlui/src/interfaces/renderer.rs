use std::marker::PhantomData;

use rsmlui_sys::render_interface::RmlRenderInterface;

use crate::interfaces::{
    HasClassPtr, InterfaceHandle, InterfaceMarker, InterfaceState, IntoRawInterface, RawInterface,
};

pub struct RenderInterfaceMarker;

impl InterfaceMarker for RenderInterfaceMarker {
    type Ptr = *mut RmlRenderInterface;
}

pub trait RenderInterface {}

impl<I> IntoRawInterface<RenderInterfaceMarker> for &mut InterfaceHandle<I>
where
    InterfaceState<I>: RenderInterface,
{
    fn into(self) -> RawInterface<RenderInterfaceMarker> {
        RawInterface(self.class_ptr() as _, PhantomData)
    }
}
