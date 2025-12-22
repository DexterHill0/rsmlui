crate::export_interfaces! {
    default: DefaultSystemInterface;

    #[cfg(feature = "system-win32")]
    win32::SystemWin32
}
