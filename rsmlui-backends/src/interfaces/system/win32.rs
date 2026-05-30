use rsmlui_core::_private::{HasOwnedInterface, not_send_sync};
use rsmlui_core::interfaces::{BorrowedInterface, IntoRawInterface, RawInterface};
use rsmlui_sys::core;
use rsmlui_sys::system_interface::{
    RmlSystemInterface, new_win32_system_interface, win32_system_interface_destructor,
};

/// Owns a C++-allocated `SystemInterface_Win32` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
pub struct SystemWin32(pub(crate) BorrowedInterface<'static, RmlSystemInterface>);

not_send_sync!(SystemWin32);

impl SystemWin32 {
    pub fn new() -> Self {
        // Safety: `new_win32_system_interface` returns a valid, non-null pointer allocated
        // with `new`. The `'static` lifetime is correct because the allocation has no
        // lifetime tied to any other resource.
        Self(unsafe { BorrowedInterface::new(new_win32_system_interface()) })
    }
}

impl Default for SystemWin32 {
    fn default() -> Self {
        Self::new()
    }
}

// Implemented on a shared borrow so the value and C++ object outlives the pointer.
impl IntoRawInterface<RmlSystemInterface> for &SystemWin32 {
    fn into_raw(self) -> RawInterface<RmlSystemInterface> {
        RawInterface::new(unsafe { self.0.as_ptr() })
    }
}

impl Drop for SystemWin32 {
    fn drop(&mut self) {
        let current = core::get_system_interface();

        let self_ptr = unsafe { self.0.as_ptr() };

        assert_ne!(
            current, self_ptr,
            "SystemWin32 dropped while still registered as RmlUI's system interface"
        );

        // Safety: pointer is valid and confirmed not currently registered above.
        unsafe { win32_system_interface_destructor(self_ptr) }
    }
}

impl HasOwnedInterface<0> for SystemWin32 {
    type Owned = Self;
}
