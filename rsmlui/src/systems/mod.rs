#[cfg(feature = "backend-win32-gl2")]
pub mod win32;
#[cfg(feature = "backend-win32-gl2")]
pub type DefaultPlatformInterface = win32::SystemWin32;
