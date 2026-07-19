use std::mem::ManuallyDrop;

use rsmlui_core::_private::{HasOwnedInterface, IntoSys, not_send_sync};
use rsmlui_core::interfaces::{BorrowedInterface, IntoRawInterface, RawInterface};
use rsmlui_core::math::UVec2;
use rsmlui_sys::interfaces::render_interface_gl2::{
    gl2_render_interface_begin_frame, gl2_render_interface_clear, gl2_render_interface_end_frame,
    gl2_render_interface_set_viewport,
};
use rsmlui_sys::render_interface::{
    RmlRenderInterface, gl2_render_interface_destructor, new_gl2_render_interface,
};
use rsmlui_sys::{Rml_Vector2i, core};

use crate::interfaces::RenderDriver;

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

impl RendererGl2 {
    /// Must be called before [`begin_frame`] whenever the window size changes.
    ///
    /// [`begin_frame`]: RendererGl2::begin_frame
    pub fn set_viewport(&mut self, dimensions: impl Into<UVec2>) {
        let dimensions: Rml_Vector2i = dimensions.into().into_sys();

        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe {
            gl2_render_interface_set_viewport(self.interface.as_ptr(), dimensions.x, dimensions.y)
        }
    }

    /// Sets up GL state for a frame. Call [`set_viewport`] first, then [`clear`], then this.
    ///
    /// [`set_viewport`]: RendererGl2::set_viewport
    /// [`clear`]: RendererGl2::clear
    pub fn begin_frame(&self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl2_render_interface_begin_frame(self.interface.as_ptr()) }
    }

    pub fn end_frame(&self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl2_render_interface_end_frame(self.interface.as_ptr()) }
    }

    /// Clears the framebuffer. Call before [`begin_frame`].
    ///
    /// [`begin_frame`]: RendererGl2::begin_frame
    pub fn clear(&self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl2_render_interface_clear(self.interface.as_ptr()) }
    }
}

impl RenderDriver for RendererGl2 {
    fn set_viewport(&mut self, dimensions: impl Into<UVec2>) {
        RendererGl2::set_viewport(self, dimensions);
    }

    fn clear(&mut self) {
        RendererGl2::clear(&self);
    }

    fn begin_frame(&mut self) {
        RendererGl2::begin_frame(&self);
    }

    fn end_frame(&mut self) {
        RendererGl2::end_frame(&self);
    }
}

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
