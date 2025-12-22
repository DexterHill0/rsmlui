// this is the only *required* feature to loosen orphan rules for when users make custom interfaces
#![feature(fundamental)]
// below are all optional, just used for convenience
// not required, the methods can easily be replicated using `.get()`
#![feature(unsafe_cell_access)]
// not required, used in core module to reduce duplication of `Backend` bounds
#![feature(trait_alias)]
// not required, can easily be replaced using `negative_impls` crate
#![feature(negative_impls)]

pub mod backends;
pub mod core;
pub mod errors;
pub mod interfaces;
pub mod platforms;
pub mod renderers;
pub mod utils;

pub use glam;
