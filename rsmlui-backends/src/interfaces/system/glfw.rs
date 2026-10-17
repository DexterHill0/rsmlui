use rsmlui_core::_private::HasOwnedInterface;
use rsmlui_core::Uncallable;
use rsmlui_core::interfaces::object::IntoRawInterface;
use rsmlui_core::object::constness::Mut;
use rsmlui_core::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject, ObjectMutPtr};
use rsmlui_core::object::ownership::{Foreign, Native, ObjectOwnership};
use rsmlui_sys::core;
use rsmlui_sys::system_interface::{
    RmlSystemInterface, glfw_system_interface_destructor, new_glfw_system_interface,
};
use rsmlui_sys::utils::c_void;

/// # Safety
///
/// - Must return a valid `GLFWwindow*` that outlives any interface built from it.
pub unsafe trait HasGlfwWindow {
    fn window_ptr(&self) -> *mut c_void;
}

#[cfg(feature = "glfw-rs")]
unsafe impl HasGlfwWindow for glfw::Window {
    fn window_ptr(&self) -> *mut c_void {
        glfw::Context::window_ptr(self).cast()
    }
}

pub mod tags {
    pub enum TagGlfw {}
}

impl ForeignObject for tags::TagGlfw {
    type Object = RmlSystemInterface;
}

unsafe impl ForeignDestroy for tags::TagGlfw {
    unsafe fn destroy(raw: *mut Self::Object) {
        unsafe { glfw_system_interface_destructor(raw) }
    }

    unsafe fn assert_droppable(raw: *mut Self::Object) {
        assert_ne!(
            core::get_system_interface(),
            raw,
            "SystemGlfw dropped while still registered as RmlUI's system interface"
        );
    }
}

/// Shared handle newtype for both [`SystemGlfw`] and [`SystemGlfwRef`].
#[repr(transparent)]
pub struct GlfwHandle<'a, M: ObjectOwnership<tags::TagGlfw>>(
    ForeignHandle<'a, tags::TagGlfw, M, Mut>,
);

/// Owns a C++-allocated `SystemInterface_GLFW` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
pub type SystemGlfw = GlfwHandle<'static, Native>;

/// A borrowed version of [`SystemGlfw`]. Identical in implementation and mutability,
/// except it does not destroy the interface on [`Drop`].
pub type SystemGlfwRef<'a> = GlfwHandle<'a, Foreign>;

impl SystemGlfw {
    pub fn new(glfw_window: &mut impl HasGlfwWindow) -> Self {
        // Safety: `new_glfw_system_interface` returns a valid, non-null pointer allocated
        // with `new`.
        Self(unsafe {
            ForeignHandle::<_, Native, Mut>::from_raw(new_glfw_system_interface(
                glfw_window.window_ptr(),
            ))
        })
    }
}

impl<'a> SystemGlfwRef<'a> {
    /// # Safety
    ///
    /// - `raw` must be a valid, non-null pointer to a `RenderInterface_GL2` that
    ///   outlives `'a`.
    pub(crate) unsafe fn from_raw(raw: ObjectMutPtr<tags::TagGlfw>) -> Self {
        Self(unsafe { ForeignHandle::<_, Foreign, Mut>::from_raw(raw) })
    }
}

impl<'a, M: ObjectOwnership<tags::TagGlfw>> IntoRawInterface<RmlSystemInterface>
    for &mut GlfwHandle<'a, M>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlSystemInterface {
        unsafe { self.0.as_mut_ptr() }
    }
}

impl<'a, M: ObjectOwnership<tags::TagGlfw>> HasOwnedInterface<RmlSystemInterface>
    for GlfwHandle<'a, M>
{
    type Owned = Self;
}
