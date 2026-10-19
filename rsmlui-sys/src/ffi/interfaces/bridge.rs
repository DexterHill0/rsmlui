use std::mem::transmute;
use std::ptr::{self, DynMetadata, Pointee};

#[cxx::bridge]
mod ffi {
    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type c_void = crate::utils::c_void;
    }

    #[namespace = "rsmlui::interfaces"]
    unsafe extern "C++" {
        include!("rsmlui/Utils.h");

        type InterfaceBridge;

        unsafe fn get_meta_from_bridge(bridge: *mut InterfaceBridge) -> *const c_void;
        unsafe fn get_data_from_bridge(bridge: *mut InterfaceBridge) -> *mut c_void;
    }
}

pub(crate) use ffi::InterfaceBridge;

/// Reconstructs a fat `*mut dyn Trait` pointer from a C++ `this` pointer whose class
/// inherits the `InterfaceBridge` class.
///
/// # Safety
///
/// - `cpp_this` must be a valid, non-null pointer whose class MUST inherit from
///   `InterfaceBridge``
/// - The `rust_meta` and `rust_data` fields must have been written by a prior call to
///   `ptr::to_raw_parts` on a fat pointer of type `*mut Trait`.
#[inline]
pub(crate) unsafe fn fat_from_cpp<CppThis, Trait>(cpp_this: *mut CppThis) -> *mut Trait
where
    Trait: ?Sized + Pointee<Metadata = DynMetadata<Trait>>,
{
    let data = unsafe { ffi::get_data_from_bridge(cpp_this.cast()) };
    let meta: DynMetadata<Trait> = unsafe { transmute(ffi::get_meta_from_bridge(cpp_this.cast())) };

    ptr::from_raw_parts_mut(data, meta)
}
