use std::mem::transmute;
use std::ptr::{self, DynMetadata, Pointee};

use crate::interfaces::InterfaceBridgeLayout;

#[macro_export]
macro_rules! const_assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        const _: [(); $left] = [(); $right];
    };
}

/// Reconstructs a fat `*mut dyn Trait` pointer from a C++ `this` pointer whose memory layout
/// matches [`InterfaceBridgeLayout`].
///
/// # Safety
///
/// - `cpp_this` must be a valid, non-null pointer whose in-memory layout matches
///   `InterfaceBridgeLayout` (vtable pointer at offset 0, `rust_meta` at offset 8,
///   `rust_data` at offset 16).
/// - The `rust_meta` and `rust_data` fields must have been written by a prior call to
///   `ptr::to_raw_parts` on a fat pointer of type `*mut Trait`.
#[inline]
pub(crate) unsafe fn fat_from_cpp<CppThis, Trait>(cpp_this: *mut CppThis) -> *mut Trait
where
    Trait: ?Sized + Pointee<Metadata = DynMetadata<Trait>>,
{
    let bridge = unsafe { &*(cpp_this as *const InterfaceBridgeLayout) };
    let meta: DynMetadata<Trait> = unsafe { transmute(bridge.rust_meta) };
    ptr::from_raw_parts_mut(bridge.rust_data, meta)
}
