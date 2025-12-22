#[cfg(feature = "backend-win32-gl2")]
pub mod s_win32_r_gl2;
#[cfg(feature = "backend-win32-gl2")]
pub type DefaultBackend = s_win32_r_gl2::BackendWin32Gl2;
