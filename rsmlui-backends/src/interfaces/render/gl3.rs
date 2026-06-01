use rsmlui_core::_private::{HasOwnedInterface, not_send_sync, not_unwind_safe};
use rsmlui_core::interfaces::{BorrowedInterface, IntoRawInterface, RawInterface};
use rsmlui_sys::core;
use rsmlui_sys::interfaces::render_interface_gl3::{
    gl3_initialize, gl3_render_interface_begin_frame, gl3_render_interface_clear,
    gl3_render_interface_end_frame, gl3_render_interface_set_viewport, gl3_shutdown,
};
use rsmlui_sys::render_interface::{
    RmlRenderInterface, gl3_render_interface_destructor, new_gl3_render_interface,
};

/// Token for constructing a [`RendererGl3`]. Holds no resources.
pub struct UninitRendererGl3 {
    _private: (),
}

not_send_sync!(UninitRendererGl3);

impl UninitRendererGl3 {
    pub fn new() -> Self {
        UninitRendererGl3 { _private: () }
    }

    /// Loads OpenGL 3.x function pointers and constructs the renderer.
    ///
    /// Returns `None` if function pointer loading fails (no current context, or insufficient
    /// OpenGL version).
    ///
    /// # Safety
    ///
    /// A valid OpenGL 3.x context must be current on this thread, and must remain current
    /// for the entire lifetime of the returned [`RendererGl3`].
    pub unsafe fn assume_init(self) -> Option<RendererGl3> {
        let mut error = String::new();

        if !gl3_initialize(&mut error) {
            rsmlui_core::error!("GL3 renderer initialization failed: {}", error);

            return None;
        }

        Some(RendererGl3 {
            // Safety: gl3_initialize() succeeded above, so glad function pointers are loaded
            // and the C++ constructor (which creates shaders) can execute safely.
            interface: unsafe { BorrowedInterface::new(new_gl3_render_interface()) },
        })
    }
}

/// Owns a C++-allocated `RenderInterface_GL3` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
///
/// # Safety
///
/// A valid OpenGL 3.x context must be current on this thread for the entire lifetime
/// that this renderer is registered with RmlUI.
///
/// To create a [`RendererGl3`], you must use [`UninitRendererGl3::assume_init`] which
/// forces the user to be aware of the safety invariant.
pub struct RendererGl3 {
    pub(crate) interface: BorrowedInterface<'static, RmlRenderInterface>,
}

not_send_sync!(RendererGl3);
not_unwind_safe!(RendererGl3);

impl RendererGl3 {
    /// Must be called before [`begin_frame`] whenever the window size changes.
    ///
    /// [`begin_frame`]: RendererGl3::begin_frame
    pub fn set_viewport(&mut self, width: i32, height: i32) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl3_render_interface_set_viewport(self.interface.as_ptr(), width, height) }
    }

    /// Sets up GL state for a frame. Call [`set_viewport`] first, then [`clear`], then this.
    ///
    /// [`set_viewport`]: RendererGl3::set_viewport
    /// [`clear`]: RendererGl3::clear
    pub fn begin_frame(&self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl3_render_interface_begin_frame(self.interface.as_ptr()) }
    }

    pub fn end_frame(&self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl3_render_interface_end_frame(self.interface.as_ptr()) }
    }

    /// Clears the framebuffer. Call before [`begin_frame`].
    ///
    /// [`begin_frame`]: RendererGl3::begin_frame
    pub fn clear(&self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl3_render_interface_clear(self.interface.as_ptr()) }
    }
}

impl IntoRawInterface<RmlRenderInterface> for &RendererGl3 {
    fn into_raw(self) -> RawInterface<RmlRenderInterface> {
        RawInterface::new(unsafe { self.interface.as_ptr() })
    }
}

impl Drop for RendererGl3 {
    fn drop(&mut self) {
        let current = core::get_render_interface();

        let self_ptr = unsafe { self.interface.as_ptr() };

        assert_ne!(
            current, self_ptr,
            "RenderInterfaceGl3 dropped while still registered as RmlUI's render interface"
        );

        // Safety: pointer is valid and confirmed not currently registered above.
        unsafe { gl3_render_interface_destructor(self_ptr) };

        gl3_shutdown();
    }
}

impl HasOwnedInterface<1> for RendererGl3 {
    type Owned = Self;
}
