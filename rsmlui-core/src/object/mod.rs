//! There are three main concepts at play for safely managing FFI objects.
//!
//! 1. "constness": see [`Constness`] for more information.
//! 2. Ownership: see [`ObjectOwnership`] for more information.
//! 3. Provenance: see [`ForeignHandle`] for more information.
//!
//! [`Constness`]: crate::object::constness::Constness
//! [`ObjectOwnership`]: crate::object::ownership::ObjectOwnership
//! [`ForeignHandle`]: crate::object::foreign::ForeignHandle

pub mod constness;
pub mod foreign;
pub mod ownership;
