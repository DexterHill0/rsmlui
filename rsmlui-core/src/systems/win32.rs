// use rsmlui_sys::system_interface::RmlSystemInterface;

// use crate::interfaces::{BorrowedInterface, RawInterface};
// use crate::not_send_sync;

// #[repr(transparent)]
// pub struct SystemWin32(pub(crate) BorrowedInterface<RmlSystemInterface>);

// not_send_sync!(SystemWin32);

// impl From<&mut SystemWin32> for RawInterface<RmlSystemInterface> {
//     fn from(val: &mut SystemWin32) -> Self {
//         RawInterface::new(val.0.raw)
//     }
// }
