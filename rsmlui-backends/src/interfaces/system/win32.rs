use rsmlui_core::_private::HasOwnedInterface;
use rsmlui_core::Uncallable;
use rsmlui_core::interfaces::object::IntoRawInterface;
use rsmlui_core::object::constness::Mut;
use rsmlui_core::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject};
use rsmlui_core::object::ownership::{Foreign, Native, ObjectOwnership};
use rsmlui_sys::core;
use rsmlui_sys::system_interface::{
    RmlSystemInterface, new_win32_system_interface, win32_system_interface_destructor,
};

pub mod tags {
    pub enum TagWin32 {}
}

impl ForeignObject for tags::TagWin32 {
    type Object = RmlSystemInterface;
}

unsafe impl ForeignDestroy for tags::TagWin32 {
    unsafe fn destroy(raw: *mut Self::Object) {
        unsafe { win32_system_interface_destructor(raw) }
    }

    unsafe fn assert_droppable(raw: *mut Self::Object) {
        assert_ne!(
            core::get_system_interface(),
            raw,
            "SystemWin32 dropped while still registered as RmlUI's system interface"
        );
    }
}

/// Shared handle newtype for both [`SystemWin32`] and [`SystemWin32Ref`].
#[repr(transparent)]
pub struct Win32Handle<'a, M: ObjectOwnership<tags::TagWin32>>(
    ForeignHandle<'a, tags::TagWin32, M, Mut>,
);

/// Owns a C++-allocated `SystemInterface_Win32` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
pub type SystemWin32 = Win32Handle<'static, Native>;

/// A borrowed version of [`SystemWin32`]. Identical in implementation and mutability,
/// except it does not destroy the interface on [`Drop`].
pub type SystemWin32Ref<'a> = Win32Handle<'a, Foreign>;

impl SystemWin32 {
    pub fn new() -> Self {
        // Safety: `new_win32_system_interface` returns a valid, non-null pointer allocated
        // with `new`.
        Self(unsafe { ForeignHandle::<_, Native, Mut>::from_raw(new_win32_system_interface()) })
    }
}

impl Default for SystemWin32 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, M: ObjectOwnership<tags::TagWin32>> IntoRawInterface<RmlSystemInterface>
    for &mut Win32Handle<'a, M>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlSystemInterface {
        unsafe { self.0.as_mut_ptr() }
    }
}

impl<'a, M: ObjectOwnership<tags::TagWin32>> HasOwnedInterface<RmlSystemInterface>
    for Win32Handle<'a, M>
{
    type Owned = Self;
}
