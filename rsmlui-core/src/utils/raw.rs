//! The following traits represent the "constness" of a given data type.
//! When we borrow a value from C++ (that is, it returns us a pointer to some data)
//! it may not necessarily be sound for that data to be mutated on the rust side.
//! For that reason, the split of [`Raw`] and [`RawMut`] determines whether a given
//! RmlUi data structure can be modified from rust. Take the following two examples:
//! 1. [`ElementDocument`]: The ownership of this value belongs to C++, as it
//!    passes us the pointer. Many methods we call on an instance of this type will modify
//!    it, so it must be both `Raw` and `RawMut`. There is no way to construct an
//!    instance in Rust directly - cpp will always be the owner - therefore the
//!    `ElementDocument` struct is always a borrowed struct, but still mutable.
//! 2. [`Variant`]: The ownership of this value is split between Rust and C++.
//!    A value can be borrowed from C++, such as borrowing a variant out of
//!    a dictionary, or it can be created from scratch and owned in Rust.
//!    Therefore, the borrowed version of a Variant is only `Raw`, NOT `RawMut`,
//!    as it is not safe to mutate a value we don't own. An instance owned by
//!    rust, however, is both `Raw` and `RawMut` because we own it and can
//!    mutate it.
//!
//! [`ElementDocument`]: crate::core::element_document::ElementDocument
//! [`Variant`]: crate::core::containers::Variant

pub(crate) trait Raw {
    type Target;

    fn raw(&self) -> *const Self::Target;
}

pub(crate) trait RawMut: Raw {
    fn raw_mut(&mut self) -> *mut Self::Target;
}

pub(crate) type Ptr<R> = *const <R as Raw>::Target;
pub(crate) type PtrMut<R> = *mut <R as Raw>::Target;
