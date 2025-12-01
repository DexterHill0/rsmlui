#[cfg(feature = "backend-win32-gl2")]
pub mod gl2;
#[cfg(feature = "backend-win32-gl2")]
pub type DefaultRenderInterface = gl2::RendererGl2;
