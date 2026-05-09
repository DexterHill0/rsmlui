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

#[cfg(feature = "backend-win32-gl2")]
pub mod win32_gl2;

use rsmlui_core::core::context::Context;
use rsmlui_core::types::input::{KeyCode, KeyModifier};
#[cfg(feature = "backend-win32-gl2")]
pub use win32_gl2::{Win32Gl2Backend, Win32Gl2BackendOptions};

pub(crate) type KeyDownCallbackDyn = dyn FnMut(&Context, KeyCode, KeyModifier, f32, bool) -> bool;
pub type KeyDownCallback = Box<KeyDownCallbackDyn>;
