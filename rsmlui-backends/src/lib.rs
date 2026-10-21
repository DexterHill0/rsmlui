// optional: can be done with `negative_impl` crate
#![feature(negative_impls)]
#![cfg_attr(any(feature = "system-winit"), feature(arbitrary_self_types))]

pub mod backend;
pub mod error;
pub mod interfaces;
pub mod monolithic;
pub mod window;
