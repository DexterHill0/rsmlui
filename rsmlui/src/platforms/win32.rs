use crate::interfaces::system::SystemInterfaceMarker;
use crate::interfaces::{BorrowedInterface, RawInterface};

#[repr(transparent)]
pub struct PlatformWin32(pub(crate) BorrowedInterface<SystemInterfaceMarker>);

impl Into<RawInterface<SystemInterfaceMarker>> for &mut PlatformWin32 {
    fn into(self) -> RawInterface<SystemInterfaceMarker> {
        RawInterface::new(self.0.raw)
    }
}
