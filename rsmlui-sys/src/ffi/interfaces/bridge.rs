use std::mem::transmute;
use std::ptr::{self, DynMetadata, Pointee};

#[cxx::bridge]
mod ffi {

    #[doc(hidden)]
    pub struct InterfaceBridgeLayout {
        pub cpp_vtable: *const c_void, // offset 0
        pub rust_meta: *const c_void,  // offset 8 (DynMetadata is a single pointer)
        pub rust_data: *mut c_void,    // offset 16
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type c_void = crate::utils::c_void;
    }
}

#[doc(hidden)]
pub use ffi::InterfaceBridgeLayout;

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
    let bridge = unsafe { &*(cpp_this.cast::<InterfaceBridgeLayout>()) };
    let meta: DynMetadata<Trait> = unsafe { transmute(bridge.rust_meta) };
    ptr::from_raw_parts_mut(bridge.rust_data, meta)
}
