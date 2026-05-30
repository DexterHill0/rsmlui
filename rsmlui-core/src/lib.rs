#![allow(incomplete_features)]
// required: `self: &mut InterfaceHandle<Self>` receivers on interfaces
#![feature(arbitrary_self_types)]
// required: `(*mut dyn Trait).to_raw_parts()` and `ptr::from_raw_parts_mut` for fat-pointer bridge
#![feature(ptr_metadata)]
// optional: can be done with `negative_impl` crate
#![feature(negative_impls)]
// optional: used for macro on `mod x;` syntax - can be worked around
#![feature(proc_macro_hygiene)]

pub mod containers;
pub mod core;
pub mod error;
pub mod interfaces;
pub mod types;
mod utils;

pub use core::backend_handle::BackendHandle;
pub use core::context::Context;
pub use core::core::Rml;
pub use core::element_document::ElementDocument;
pub use core::log;
pub use core::log::{always, assert, debug, error, info, warning};

pub use error::{Error, Result};
pub use types::math;

#[doc(hidden)]
pub mod _private {
    pub use crate::interfaces::_private::HasOwnedInterface;
    pub use crate::utils::conversions::{FromSys, IntoSys};
    pub use crate::utils::macros::not_send_sync;
}
