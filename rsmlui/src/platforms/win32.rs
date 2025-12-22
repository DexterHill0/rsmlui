use crate::interfaces::system::SystemInterfaceMarker;
use crate::interfaces::{BorrowedInterface, RawInterface};
use crate::not_send_sync;

#[repr(transparent)]
pub struct PlatformWin32(pub(crate) BorrowedInterface<SystemInterfaceMarker>);

not_send_sync!(PlatformWin32);

impl Into<RawInterface<SystemInterfaceMarker>> for &mut PlatformWin32 {
    fn into(self) -> RawInterface<SystemInterfaceMarker> {
        RawInterface::new(self.0.raw)
    }
}
