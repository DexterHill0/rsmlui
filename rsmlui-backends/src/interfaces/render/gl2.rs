use std::mem::ManuallyDrop;

use rsmlui_core::_private::{HasOwnedInterface, not_send_sync};
use rsmlui_core::interfaces::{BorrowedInterface, IntoRawInterface, RawInterface};
use rsmlui_sys::core;
use rsmlui_sys::render_interface::{
    RmlRenderInterface, gl2_render_interface_destructor, new_gl2_render_interface,
};

/// Uninitialized GL2 renderer. Holds the C++ object but is not yet usable as a render interface.
///
/// See [`UninitRendererGl2::assume_init`].
pub struct UninitRendererGl2 {
    interface: BorrowedInterface<'static, RmlRenderInterface>,
}

not_send_sync!(UninitRendererGl2);

impl UninitRendererGl2 {
    /// Constructs the C++ `RenderInterface_GL2`. Safe because the GL2 constructor makes no GL calls.
    pub fn new() -> Self {
        Self {
            interface: unsafe { BorrowedInterface::new(new_gl2_render_interface()) },
        }
    }

    /// Marks this renderer as ready to use as a render interface.
    ///
    /// # Safety
    ///
    /// A valid OpenGL context must be current on this thread, and must remain current
    /// for the entire lifetime of the returned [`RendererGl2`].
    pub unsafe fn assume_init(self) -> RendererGl2 {
        let me = ManuallyDrop::new(self);
        RendererGl2 {
            // Safety: `me` will not be dropped (ManuallyDrop), so ownership of the
            // interface pointer transfers cleanly to `RendererGl2`.
            interface: unsafe { std::ptr::read(&me.interface) },
        }
    }
}

impl Drop for UninitRendererGl2 {
    fn drop(&mut self) {
        unsafe { gl2_render_interface_destructor(self.interface.as_ptr()) }
    }
}

/// Owns a C++-allocated `RenderInterface_GL2` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
///
/// # Safety invariant
///
/// A valid OpenGL context must be current on this thread for the entire
/// that this renderer is registered with RmlUI.
pub struct RendererGl2 {
    pub(crate) interface: BorrowedInterface<'static, RmlRenderInterface>,
}

not_send_sync!(RendererGl2);

impl IntoRawInterface<RmlRenderInterface> for &RendererGl2 {
    fn into_raw(self) -> RawInterface<RmlRenderInterface> {
        RawInterface::new(unsafe { self.interface.as_ptr() })
    }
}

impl Drop for RendererGl2 {
    fn drop(&mut self) {
        let current = core::get_render_interface();

        let self_ptr = unsafe { self.interface.as_ptr() };

        assert_ne!(
            current, self_ptr,
            "RenderInterfaceGl2 dropped while still registered as RmlUI's render interface"
        );

        // Safety: pointer is valid and confirmed not currently registered above.
        unsafe { gl2_render_interface_destructor(self_ptr) }
    }
}

impl HasOwnedInterface<1> for RendererGl2 {
    type Owned = Self;
}
