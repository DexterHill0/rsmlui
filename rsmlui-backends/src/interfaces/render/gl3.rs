use rsmlui_core::_private::{HasOwnedInterface, IntoSys, not_send_sync, not_unwind_safe};
use rsmlui_core::Uncallable;
use rsmlui_core::interfaces::object::IntoRawInterface;
use rsmlui_core::math::UVec2;
use rsmlui_core::object::constness::Mut;
use rsmlui_core::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject};
use rsmlui_core::object::ownership::{Foreign, Native, ObjectOwnership};
use rsmlui_sys::interfaces::render_interface_gl3::{
    gl3_initialize, gl3_render_interface_begin_frame, gl3_render_interface_clear,
    gl3_render_interface_end_frame, gl3_render_interface_set_viewport, gl3_shutdown,
};
use rsmlui_sys::render_interface::{
    RmlRenderInterface, gl3_render_interface_destructor, new_gl3_render_interface,
};
use rsmlui_sys::{Rml_Vector2i, core};

pub mod tags {
    pub enum TagGl3 {}
}

impl ForeignObject for tags::TagGl3 {
    type Object = RmlRenderInterface;
}

unsafe impl ForeignDestroy for tags::TagGl3 {
    unsafe fn destroy(raw: *mut Self::Object) {
        // Safety: the pointer is valid and non-null
        unsafe { gl3_render_interface_destructor(raw) };

        // the glad function pointers loaded by `assume_init` are no longer needed
        // once the renderer that uses them is gone
        gl3_shutdown();
    }

    unsafe fn assert_droppable(raw: *mut Self::Object) {
        assert_ne!(
            core::get_render_interface(),
            raw,
            "RendererGl3 dropped while still registered as RmlUI's render interface"
        );
    }
}

/// Token for constructing a [`RendererGl3`]. Holds no resources.
#[derive(Default)]
pub struct UninitRendererGl3 {
    _private: (),
}

not_send_sync!(UninitRendererGl3);

/// Shared handle newtype for both [`RendererGl3`] and [`RendererGl3Ref`].
#[repr(transparent)]
pub struct Gl3Handle<'a, M: ObjectOwnership<tags::TagGl3>>(ForeignHandle<'a, tags::TagGl3, M, Mut>);

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
pub type RendererGl3 = Gl3Handle<'static, Native>;

/// A borrowed version of [`RendererGl3`]. Identical in implementation and mutability,
/// except it does not destroy the renderer on [`Drop`].
pub type RendererGl3Ref<'a> = Gl3Handle<'a, Foreign>;

not_unwind_safe!(['a, M: ObjectOwnership<tags::TagGl3>] Gl3Handle['a, M]);

impl UninitRendererGl3 {
    /// Creates a new `UninitRendererGl3`. Does not invoke any foreign code until
    /// [`UninitRendererGl3::assume_init`] is called.
    pub fn new() -> Self {
        Self::default()
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

        // Safety: gl3_initialize() succeeded above, so glad function pointers are loaded
        // and the C++ constructor (which creates shaders) can execute safely.
        Some(Gl3Handle(unsafe {
            ForeignHandle::<_, Native, Mut>::from_raw(new_gl3_render_interface())
        }))
    }
}

impl<'a, M: ObjectOwnership<tags::TagGl3>> Gl3Handle<'a, M> {
    /// Must be called before [`begin_frame`] whenever the window size changes.
    ///
    /// [`begin_frame`]: RendererGl3::begin_frame
    pub fn set_viewport(&mut self, dimensions: impl Into<UVec2>) {
        let dimensions: Rml_Vector2i = dimensions.into().into_sys();

        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe {
            gl3_render_interface_set_viewport(self.0.as_mut_ptr(), dimensions.x, dimensions.y)
        }
    }

    /// Sets up GL state for a frame. Call [`set_viewport`] first, then [`clear`], then this.
    ///
    /// [`set_viewport`]: RendererGl3::set_viewport
    /// [`clear`]: RendererGl3::clear
    pub fn begin_frame(&mut self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl3_render_interface_begin_frame(self.0.as_mut_ptr()) }
    }

    pub fn end_frame(&mut self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl3_render_interface_end_frame(self.0.as_mut_ptr()) }
    }

    /// Clears the framebuffer. Call before [`begin_frame`].
    ///
    /// [`begin_frame`]: RendererGl3::begin_frame
    pub fn clear(&mut self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL3` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl3_render_interface_clear(self.0.as_mut_ptr()) }
    }
}

impl<'a, M: ObjectOwnership<tags::TagGl3>> IntoRawInterface<RmlRenderInterface>
    for &mut Gl3Handle<'a, M>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlRenderInterface {
        unsafe { self.0.as_mut_ptr() }
    }
}

impl<'a, M: ObjectOwnership<tags::TagGl3>> HasOwnedInterface<RmlRenderInterface>
    for Gl3Handle<'a, M>
{
    type Owned = Self;
}
