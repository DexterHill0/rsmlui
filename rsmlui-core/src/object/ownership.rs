use sealed::sealed;

use crate::object::foreign::{ForeignDestroy, ForeignObject};

/// This represents who (Rust - [`Native`] or C++ - [`Foreign`]) a given object
/// belongs to. This is different to the "constness" and provenance of an object.
///
/// - A [`Native`] object means the memory is managed by us, and we are responsible
///   for using the memory safely, and making sure it is cleaned up when the object
///   is dropped.
///
/// - A [`Foreign`] object is one that we borrow from RmlUi. We do not control the
///   memory and do not (and should not) do anything to the value on drop, or we risk
///   memory corruption, UAFs, etc.
#[sealed]
pub trait ObjectOwnership<O: ForeignObject> {
    /// Called when a [`ForeignHandle`] is dropped.
    ///
    /// # Safety
    ///
    /// If the handle is:
    /// - `Foreign` (borrowed): this should be a noop.
    /// - `Native` (owned): this should uphold any invariants required
    ///   when dropping `O::Object`.
    ///
    /// [`ForeignHandle`]: crate::object::foreign::ForeignHandle
    unsafe fn on_drop(raw: *mut O::Object);
}

/// Rust owns the C++ resource and destroys it on drop.
#[derive(Debug)]
pub enum Native {}

/// C++ owns the resource therfore there is no drop code.
#[derive(Debug)]
pub enum Foreign {}

#[sealed]
impl<O: ForeignObject> ObjectOwnership<O> for Foreign {
    // We don't want to run any drop code for a borrowed value as that could lead to
    // memory corruption, UAFs, etc.
    unsafe fn on_drop(_raw: *mut O::Object) {}
}

#[sealed]
impl<O: ForeignDestroy> ObjectOwnership<O> for Native {
    unsafe fn on_drop(raw: *mut O::Object) {
        unsafe { O::assert_droppable(raw) };

        unsafe { O::destroy(raw) }
    }
}
