use rsmlui_core::_private::{HasOwnedInterface, not_send_sync};
use rsmlui_core::interfaces::{BorrowedInterface, IntoRawInterface, RawInterface};
use rsmlui_sys::core;
use rsmlui_sys::interfaces::Opaque;
use rsmlui_sys::system_interface::{
    RmlSystemInterface, glfw_system_interface_destructor, new_glfw_system_interface,
};

/// # Safety
///
/// - Must return a valid `GLFWwindow*` that outlives any interface built from it.
pub unsafe trait HasGlfwWindow {
    fn window_ptr(&self) -> *mut Opaque;
}

#[cfg(feature = "glfw-rs")]
unsafe impl HasGlfwWindow for glfw::Window {
    fn window_ptr(&self) -> *mut Opaque {
        glfw::Context::window_ptr(self).cast()
    }
}

/// Owns a C++-allocated `SystemInterface_GLFW` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
pub struct SystemGlfw(pub(crate) BorrowedInterface<'static, RmlSystemInterface>);

not_send_sync!(SystemGlfw);

impl SystemGlfw {
    pub fn new(glfw_window: &mut impl HasGlfwWindow) -> Self {
        // Safety: `new_gflw_system_interface` returns a valid, non-null pointer allocated
        // with `new`. The `'static` lifetime is correct because the allocation has no
        // lifetime tied to any other resource.
        Self(unsafe { BorrowedInterface::new(new_glfw_system_interface(glfw_window.window_ptr())) })
    }
}

// Implemented on a shared borrow so the value and C++ object outlives the pointer.
impl IntoRawInterface<RmlSystemInterface> for &SystemGlfw {
    fn into_raw(self) -> RawInterface<RmlSystemInterface> {
        RawInterface::new(unsafe { self.0.as_ptr() })
    }
}

impl Drop for SystemGlfw {
    fn drop(&mut self) {
        let current = core::get_system_interface();

        let self_ptr = unsafe { self.0.as_ptr() };

        assert_ne!(
            current, self_ptr,
            "SystemGlfw dropped while still registered as RmlUI's system interface"
        );

        // Safety: pointer is valid and confirmed not currently registered above.
        unsafe { glfw_system_interface_destructor(self_ptr) }
    }
}

impl HasOwnedInterface<0> for SystemGlfw {
    type Owned = Self;
}
