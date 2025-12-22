crate::export_interfaces! {
    default: DefaultBackend;

    #[cfg(feature = "backend-win32-gl2")]
    s_win32_r_gl2::BackendWin32Gl2
}
