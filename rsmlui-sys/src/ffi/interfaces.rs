/// Mirrors the memory layout of the C++ `RustSystemInterface` (and other interface) classes.
/// Used by `fat_from_cpp` to reconstruct fat pointers from a C++ `this` pointer.
///
/// Static asserts in each interface rust module verify that the C++ struct fields sit at the
/// exact same offsets as the fields here.
///
/// This is driven by bindgen generating a representation of the C++ class which the asserts
/// assert against. If the C++ class were to change, bindgen would generate different bindings
/// making the asserts fail.
#[doc(hidden)]
#[repr(C)]
pub struct InterfaceBridgeLayout {
    pub cpp_vtable: *const (), // offset 0
    pub rust_meta: *const (),  // offset 8 (DynMetadata is a single pointer)
    pub rust_data: *mut (),    // offset 16
}

/// Opaque zero-sized type used as the pointee for the `void*` parameters in
/// `new_rust_system_interface` and similar C++ constructor helpers. CXX
/// doesn't allow `*mut ()` pointers, so insteas we use `*mut Opaque`.
pub struct Opaque {
    _private: [u8; 0],
}

unsafe impl cxx::ExternType for Opaque {
    type Id = cxx::type_id!("rsmlui::Opaque");
    type Kind = cxx::kind::Opaque;
}
