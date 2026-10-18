use rsmlui_core::_private::{HasOwnedInterface, IntoSys};
use rsmlui_core::Uncallable;
use rsmlui_core::interfaces::object::IntoRawInterface;
use rsmlui_core::math::UVec2;
use rsmlui_core::object::constness::NonConst;
use rsmlui_core::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject, ObjectMutPtr};
use rsmlui_core::object::ownership::{Foreign, Native, ObjectOwnership};
use rsmlui_sys::interfaces::render_interface_gl2::{
    gl2_render_interface_begin_frame, gl2_render_interface_clear, gl2_render_interface_end_frame,
    gl2_render_interface_set_viewport,
};
use rsmlui_sys::render_interface::{
    RmlRenderInterface, gl2_render_interface_destructor, new_gl2_render_interface,
};
use rsmlui_sys::{Rml_Vector2i, core};

pub mod tags {
    pub enum TagGl2Uninit {}
    pub enum TagGl2 {}
}

impl ForeignObject for tags::TagGl2Uninit {
    type Object = RmlRenderInterface;
}

unsafe impl ForeignDestroy for tags::TagGl2Uninit {
    unsafe fn destroy(raw: *mut Self::Object) {
        // Safety: the user has not called `assume_init` yet, which means we can
        // assume A) an OpenGl context has not been intialized and B) this renderer
        // is not yet installed, therefore dropping in its current state is safe
        unsafe { gl2_render_interface_destructor(raw) }
    }
}

impl ForeignObject for tags::TagGl2 {
    type Object = RmlRenderInterface;
}

unsafe impl ForeignDestroy for tags::TagGl2 {
    unsafe fn destroy(raw: *mut Self::Object) {
        unsafe { gl2_render_interface_destructor(raw) }
    }

    unsafe fn assert_droppable(raw: *mut Self::Object) {
        assert_ne!(
            core::get_render_interface(),
            raw,
            "RendererGl2 dropped while still registered as RmlUI's render interface"
        );
    }
}

/// Uninitialized GL2 renderer. Holds the C++ object but is not yet usable as a render interface.
/// See [`RendererGl2Uninit::assume_init`].
#[repr(transparent)]
pub struct RendererGl2Uninit(ForeignHandle<'static, tags::TagGl2Uninit, Native, NonConst>);

/// Shared handle newtype for both [`RendererGl2`] and [`RendererGl2Ref`].
#[repr(transparent)]
pub struct Gl2Handle<'a, M: ObjectOwnership<tags::TagGl2>>(ForeignHandle<'a, tags::TagGl2, M, NonConst>);

/// Owns a C++-allocated `RenderInterface_GL2` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
///
/// # Safety
///
/// A valid OpenGL context must be current on this thread for the entire
/// that this renderer is registered with RmlUI.
pub type RendererGl2 = Gl2Handle<'static, Native>;

/// A borrowed version of [`RendererGl2`]. Identical in implementation and mutability,
/// except it does not destroy the renderer on [`Drop`].
pub type RendererGl2Ref<'a> = Gl2Handle<'a, Foreign>;

impl Default for RendererGl2Uninit {
    fn default() -> Self {
        // Safety: new_gl2_render_interface returns a valid, non-null `RenderInterface_GL2`
        unsafe {
            Self(ForeignHandle::<_, Native, NonConst>::from_raw(
                new_gl2_render_interface(),
            ))
        }
    }
}

impl RendererGl2Uninit {
    /// Constructs the C++ `RenderInterface_GL2`. Safe because the GL2 constructor makes no GL calls.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks this renderer as ready to use as a render interface.
    ///
    /// # Safety
    ///
    /// A valid OpenGL context must be current on this thread, and must remain current
    /// for the entire lifetime of the returned [`RendererGl2`].
    pub unsafe fn assume_init(self) -> RendererGl2 {
        // Safety: by calling this function the user is ensuring the above invariants
        // are respected.
        Gl2Handle(unsafe { self.0.retag::<tags::TagGl2>() })
    }
}

impl<'a, M: ObjectOwnership<tags::TagGl2>> Gl2Handle<'a, M> {
    /// Must be called before [`begin_frame`] whenever the window size changes.
    ///
    /// [`begin_frame`]: RendererGl2::begin_frame
    pub fn set_viewport(&mut self, dimensions: impl Into<UVec2>) {
        let dimensions: Rml_Vector2i = dimensions.into().into_sys();

        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe {
            gl2_render_interface_set_viewport(self.0.as_mut_ptr(), dimensions.x, dimensions.y)
        }
    }

    /// Sets up GL state for a frame. Call [`set_viewport`] first, then [`clear`], then this.
    ///
    /// [`set_viewport`]: RendererGl2::set_viewport
    /// [`clear`]: RendererGl2::clear
    pub fn begin_frame(&mut self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl2_render_interface_begin_frame(self.0.as_mut_ptr()) }
    }

    pub fn end_frame(&mut self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl2_render_interface_end_frame(self.0.as_mut_ptr()) }
    }

    /// Clears the framebuffer. Call before [`begin_frame`].
    ///
    /// [`begin_frame`]: RendererGl2::begin_frame
    pub fn clear(&mut self) {
        // Safety: `interface` is a valid, non-null `RenderInterface_GL2` and there is a
        // valid GL context alive, due to the invariant of `assume_init`.
        unsafe { gl2_render_interface_clear(self.0.as_mut_ptr()) }
    }
}

impl<'a> RendererGl2Ref<'a> {
    /// # Safety
    ///
    /// - `raw` must be a valid, non-null pointer to a `RenderInterface_GL2` that
    ///   outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: ObjectMutPtr<tags::TagGl2>) -> Self {
        Self(unsafe { ForeignHandle::<_, Foreign, NonConst>::from_raw(raw) })
    }
}

impl<'a, M: ObjectOwnership<tags::TagGl2>> IntoRawInterface<RmlRenderInterface>
    for &mut Gl2Handle<'a, M>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlRenderInterface {
        unsafe { self.0.as_mut_ptr() }
    }
}

impl<'a, M: ObjectOwnership<tags::TagGl2>> HasOwnedInterface<RmlRenderInterface>
    for Gl2Handle<'a, M>
{
    type Owned = Self;
}
