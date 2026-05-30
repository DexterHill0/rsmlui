use rsmlui_core::_private::{HasOwnedInterface, not_send_sync};
use rsmlui_core::interfaces::{BorrowedInterface, IntoRawInterface, RawInterface};
use rsmlui_sys::core;
use rsmlui_sys::file_interface::{
    RmlFileInterface, default_file_interface_destructor, new_default_file_interface,
};

/// Owns a C++-allocated `DefaultFileInterface` instance.
///
/// Dropping while the interface is still registered with RmlUI will panic.
pub struct FileDefault(pub(crate) BorrowedInterface<'static, RmlFileInterface>);

not_send_sync!(FileDefault);

impl FileDefault {
    pub fn new() -> Self {
        // Safety: `new_default_file_interface` returns a valid, non-null pointer allocated
        // with `new`. The `'static` lifetime is correct because the allocation has no
        // lifetime tied to any other resource.
        Self(unsafe { BorrowedInterface::new(new_default_file_interface()) })
    }
}

impl Default for FileDefault {
    fn default() -> Self {
        Self::new()
    }
}

// Implemented on a shared borrow so the value and C++ object outlives the pointer.
impl IntoRawInterface<RmlFileInterface> for &FileDefault {
    fn into_raw(self) -> RawInterface<RmlFileInterface> {
        RawInterface::new(unsafe { self.0.as_ptr() })
    }
}

impl Drop for FileDefault {
    fn drop(&mut self) {
        let current = core::get_file_interface();

        let self_ptr = unsafe { self.0.as_ptr() };

        assert_ne!(
            current, self_ptr,
            "FileDefault dropped while still registered as RmlUI's fil interface"
        );

        // Safety: pointer is valid and confirmed not currently registered above.
        unsafe { default_file_interface_destructor(self_ptr) }
    }
}

impl HasOwnedInterface<2> for FileDefault {
    type Owned = Self;
}
