use rsmlui_core::_private::HasOwnedInterface;
use rsmlui_core::Uncallable;
use rsmlui_core::interfaces::object::IntoRawInterface;
use rsmlui_core::object::constness::Mut;
use rsmlui_core::object::foreign::{ForeignDestroy, ForeignHandle, ForeignObject};
use rsmlui_core::object::ownership::{Foreign, Native, ObjectOwnership};
use rsmlui_sys::core;
use rsmlui_sys::file_interface::{
    RmlFileInterface, default_file_interface_destructor, new_default_file_interface,
};

pub mod tags {
    pub enum TagFileDefault {}
}

impl ForeignObject for tags::TagFileDefault {
    type Object = RmlFileInterface;
}

unsafe impl ForeignDestroy for tags::TagFileDefault {
    unsafe fn destroy(raw: *mut Self::Object) {
        unsafe { default_file_interface_destructor(raw) }
    }

    unsafe fn assert_droppable(raw: *mut Self::Object) {
        assert_ne!(
            core::get_file_interface(),
            raw,
            "FileDefault dropped while still registered as RmlUI's file interface"
        );
    }
}

/// Shared handle newtype for both [`FileDefault`] and [`FileDefaultRef`].
#[repr(transparent)]
pub struct FileDefaultHandle<'a, M: ObjectOwnership<tags::TagFileDefault>>(
    ForeignHandle<'a, tags::TagFileDefault, M, Mut>,
);

/// Owns a C++-allocated `DefaultFileInterface` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
pub type FileDefault = FileDefaultHandle<'static, Native>;

/// A borrowed version of [`FileDefault`]. Identical in implementation and mutability,
/// except it does not destroy the interface on [`Drop`].
pub type FileDefaultRef<'a> = FileDefaultHandle<'a, Foreign>;

impl FileDefault {
    pub fn new() -> Self {
        // Safety: `new_default_file_interface` returns a valid, non-null pointer allocated
        // with `new`.
        Self(unsafe { ForeignHandle::<_, Native, Mut>::from_raw(new_default_file_interface()) })
    }
}

impl Default for FileDefault {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, M: ObjectOwnership<tags::TagFileDefault>> IntoRawInterface<RmlFileInterface>
    for &mut FileDefaultHandle<'a, M>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlFileInterface {
        unsafe { self.0.as_mut_ptr() }
    }
}

impl<'a, M: ObjectOwnership<tags::TagFileDefault>> HasOwnedInterface<RmlFileInterface>
    for FileDefaultHandle<'a, M>
{
    type Owned = Self;
}
