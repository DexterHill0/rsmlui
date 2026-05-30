use rsmlui_core::_private::not_send_sync;
use rsmlui_core::interfaces::{BorrowedInterface, IntoRawInterface, RawInterface};
use rsmlui_sys::core;
use rsmlui_sys::render_interface::{
    RmlRenderInterface, gl2_render_interface_destructor, new_gl2_render_interface,
};

/// Owns a C++-allocated `RenderInterface_GL2` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
pub struct RendererGl2(pub(crate) BorrowedInterface<'static, RmlRenderInterface>);

not_send_sync!(RendererGl2);

impl RendererGl2 {
    pub fn new() -> Self {
        // Safety: `new_gl2_render_interface` returns a valid, non-null pointer allocated
        // with `new`. The `'static` lifetime is correct because the allocation has no
        // lifetime tied to any other resource.
        Self(unsafe { BorrowedInterface::new(new_gl2_render_interface()) })
    }
}

impl Default for RendererGl2 {
    fn default() -> Self {
        Self::new()
    }
}

// Implemented on a shared borrow so the value and C++ object outlives the pointer.
impl IntoRawInterface<RmlRenderInterface> for &RendererGl2 {
    fn into_raw(self) -> RawInterface<RmlRenderInterface> {
        RawInterface::new(unsafe { self.0.as_ptr() })
    }
}

impl Drop for RendererGl2 {
    fn drop(&mut self) {
        let current = core::get_render_interface();

        let self_ptr = unsafe { self.0.as_ptr() };

        assert_ne!(
            current, self_ptr,
            "RenderInterfaceGl2 dropped while still registered as RmlUI's render interface"
        );

        // Safety: pointer is valid and confirmed not currently registered above.
        unsafe { gl2_render_interface_destructor(self_ptr) }
    }
}
