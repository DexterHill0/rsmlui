use crate::interfaces::system::{IntoSystemInterfacePtr, SystemInterface};

pub struct PlatformWin32 {
    pub(crate) raw: *mut rsmlui_sys::system_interface::SystemInterface,
}

impl IntoSystemInterfacePtr for PlatformWin32 {
    fn into_ptr(self) -> *mut rsmlui_sys::system_interface::SystemInterface {
        self.raw
    }
}
