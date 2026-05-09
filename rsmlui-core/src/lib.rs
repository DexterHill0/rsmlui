// required: `self: &mut InterfaceHandle<Self>` receivers on interfaces
#![feature(arbitrary_self_types)]
// required: `(*mut dyn Trait).to_raw_parts()` and `ptr::from_raw_parts_mut` for fat-pointer bridge
#![feature(ptr_metadata)]
// optional: can be done with `negative_impl` crate
#![feature(negative_impls)]

pub mod core;
pub mod errors;
pub mod interfaces;
pub mod renderers;
pub mod systems;
pub mod types;
mod utils;

pub use core::backend_handle::BackendHandle;

pub use glam;
pub use utils::conversions::{FromSys, IntoSys};
