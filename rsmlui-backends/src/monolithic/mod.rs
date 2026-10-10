//! Opaque wrappers around RmlUi's built-in C++ backends.
//!
//! Each variant is a monolithic unit. The windowing code, system interface, and render
//! interface are all managed by C++ and cannot be individually swapped out from Rust.
//! Enable the relevamt Cargo feature to compile in the C++ backend code:
//!
//! | Feature                | Supports              |
//! |------------------------|-----------------------|
//! | `backend-win32-gl2`    | Win32 + OpenGL 2      |
//!
//! For fully-customisable backends, use [`Backend`] together with a [`WindowDriver`]
//! implementation instead.
//!
//! [`Backend`]: crate::backend::Backend
//! [`WindowDriver`]: crate::window::WindowDriver

use rsmlui_core::core::context::Context;
use rsmlui_core::types::input::{KeyCode, KeyModifier};

#[cfg(all(feature = "system-glfw", feature = "renderer-gl2"))]
pub mod glfw_gl2;
#[cfg(all(target_os = "windows", feature = "renderer-gl2"))]
pub mod win32_gl2;

#[cfg(all(feature = "system-glfw", feature = "renderer-gl2"))]
pub use glfw_gl2::GlfwGl2Backend;
#[cfg(all(target_os = "windows", feature = "renderer-gl2"))]
pub use win32_gl2::Win32Gl2Backend;

pub(crate) type KeyDownCallbackDyn = dyn FnMut(&Context, KeyCode, KeyModifier, f32, bool) -> bool;
pub type KeyDownCallback = Box<KeyDownCallbackDyn>;

#[derive(Debug, Clone, Copy)]
pub struct BackendOptions {
    pub allow_resize: bool,
    pub power_save: bool,
}

impl Default for BackendOptions {
    fn default() -> Self {
        Self {
            allow_resize: true,
            power_save: false,
        }
    }
}
