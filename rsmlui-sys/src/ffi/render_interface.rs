use std::mem::offset_of;

use cxx::{ExternType, type_id};

use crate::interfaces::InterfaceBridgeLayout;
use crate::utils::fat_from_cpp;
use crate::{Layouts_RenderInterfaceLayoutGuard, const_assert_eq};

unsafe impl ExternType for crate::Rml_Vertex {
    type Id = type_id!("Rml::Vertex");
    type Kind = cxx::kind::Trivial;
}

// Asserts that the layout of the interface bridge matches the layout of the
// `RustRenderInterface` struct in C++.
const _: () = {
    const_assert_eq!(
        offset_of!(Layouts_RenderInterfaceLayoutGuard, vtable_),
        offset_of!(InterfaceBridgeLayout, cpp_vtable)
    );
    const_assert_eq!(
        offset_of!(Layouts_RenderInterfaceLayoutGuard, rust_meta),
        offset_of!(InterfaceBridgeLayout, rust_meta)
    );
    const_assert_eq!(
        offset_of!(Layouts_RenderInterfaceLayoutGuard, rust_data),
        offset_of!(InterfaceBridgeLayout, rust_data)
    );
};

pub unsafe trait RenderInterfaceBridge {
    unsafe fn compile_geometry(
        &mut self,
        vertices: &[Vertex],
        indices: &[i32],
    ) -> CompiledGeometryHandle;
}

unsafe fn rust_compile_geometry(
    cpp_this: *mut RustRenderInterface,
    vertices: &[Vertex],
    indices: &[i32],
) -> CompiledGeometryHandle {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this))
            .compile_geometry(vertices, indices)
    }
}

#[cxx::bridge]
mod ffi {

    #[namespace = "Rml"]
    extern "C++" {
        #[cxx_name = "RenderInterface"]
        type RmlRenderInterface;

        type Vertex = crate::Rml_Vertex;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/InterfaceDecls.h");

        type Opaque = crate::interfaces::Opaque;
    }

    #[namespace = "rsmlui::render_interface"]
    unsafe extern "C++" {
        include!("rsmlui/RenderInterface.h");

        type RustRenderInterface;

        /// # Safety
        ///
        /// - `interface_meta` and `interface_data` must be valid pointers constructed from `to_raw_parts`.
        unsafe fn new_rust_render_interface(
            interface_meta: *const Opaque,
            interface_data: *mut Opaque,
        ) -> *mut RustRenderInterface;

        /// # Safety
        ///
        /// - `obj` must be a valid, non-null pointer to a `RustRenderInterface`.
        unsafe fn rust_render_interface_destructor(obj: *mut RustRenderInterface);

        // Calls the base `Rml::RenderInterface` implementation directly, bypassing any override.
        // Used by the default method implementations on the safe crate's `RenderInterface` trait.
        #[doc(hidden)]
        unsafe fn render_interface_default_compile_geometry(
            ptr: *mut RustRenderInterface,
            vertices: &[Vertex],
            indices: &[i32],
        ) -> usize; // `CompiledGeometryHandle`
    }

    extern "Rust" {
        unsafe fn rust_compile_geometry(
            cpp_this: *mut RustRenderInterface,
            vertices: &[Vertex],
            indices: &[i32],
        ) -> usize;
    }
}

pub use ffi::{
    RmlRenderInterface, RustRenderInterface, Vertex, new_rust_render_interface,
    render_interface_default_compile_geometry, rust_render_interface_destructor,
};

pub type CompiledGeometryHandle = usize;
