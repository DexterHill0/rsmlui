use std::mem::transmute;

use rsmlui_macros::rmldoc;
use rsmlui_sys::core;
use rsmlui_sys::interfaces::Opaque;
use rsmlui_sys::render_interface::{
    RenderInterfaceBridge, RmlRenderInterface, RustRenderInterface, new_rust_render_interface,
    render_interface_default_compile_geometry, rust_render_interface_destructor,
};
use sealed::sealed;

use crate::interfaces::{InterfaceHandle, IntoRawInterface, OwnedInterface, RawInterface};
use crate::types::renderer::{CompiledGeometryHandle, Vertex};
use crate::utils::conversions::{FromSys, IntoSys};

/// The receiver type for all [`RenderInterface`] methods.
pub type RenderInterfaceHandle<T> = InterfaceHandle<T, RustRenderInterface>;

/// An owned, heap-pinned render interface. Construct via [`OwnedInterface::new`].
pub type OwnedRenderInterface<T> = OwnedInterface<T, RustRenderInterface>;

/// Implement this trait to create a custom [`Rml::RenderInterface`] for RmlUi.
///
/// Every method has a default implementation that forwards to the C++ base class behaviour.
///
/// Methods receive `self: &mut RenderInterfaceHandle<Self>` rather than `&mut self` in order
/// to ensure correct initialisation and lifetime of values.
/// [`RenderInterfaceHandle`] implements [`Deref`] and [`DerefMut`] to give access to the
/// underlying user data.
///
/// ## Dyn-compatibility
///
/// Every method is marked `where Self: Sized`, which keeps the trait dyn-compatible. Therefore,
/// methods are not callable through a `dyn RenderInterface` trait object. Dispatch must always
/// go through the [`InterfaceHandle`].
///
/// [`Deref`]: std::ops::Deref
/// [`DerefMut`]: std::ops::DerefMut
/// [`Rml::RenderInterface`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/renderer.html
#[rmldoc(file = "api_Rml-RenderInterface.md", name = "Rml::RenderInterface")]
pub trait RenderInterface {
    // #[rmldoc(name = "Rml::RenderInterface::CompileGeometry")]
    fn compile_geometry(
        self: &mut RenderInterfaceHandle<Self>,
        vertices: &[Vertex],
        indices: &[i32],
    ) -> CompiledGeometryHandle
    where
        Self: Sized,
    {
        unsafe {
            render_interface_default_compile_geometry(
                self.bridge_ptr(),
                vertices.into_sys(),
                indices,
            )
        }
    }
}

// The sys crate uses `RenderInterfaceBridge` as the dispatch from C++ to Rust.
// This impl forwards each call to the user's `RenderInterface` implementation.
unsafe impl<T: RenderInterface> RenderInterfaceBridge for RenderInterfaceHandle<T> {
    #[inline]
    unsafe fn compile_geometry(
        &mut self,
        vertices: &[rsmlui_sys::Rml_Vertex],
        indices: &[i32],
    ) -> CompiledGeometryHandle {
        T::compile_geometry(self, FromSys::from_sys(vertices), indices)
    }
}

#[sealed]
impl<T: RenderInterface> super::OwnedInterfaceHandle<RustRenderInterface> for T {
    fn init_bridge(handle: &mut RenderInterfaceHandle<T>) {
        // The fat pointer data component is the address of the heap-allocated InterfaceHandle.
        // That address is stable for the lifetime of the OwnedInterface.
        let fat_ptr: *mut dyn RenderInterfaceBridge = handle;

        let (data, meta) = fat_ptr.to_raw_parts();

        let meta_raw: *const () = unsafe { transmute(meta) };

        let cpp =
            unsafe { new_rust_render_interface(meta_raw as *const Opaque, data as *mut Opaque) };

        handle.bridge = cpp;
    }

    unsafe fn destroy(handle: &mut RenderInterfaceHandle<T>) {
        unsafe { rust_render_interface_destructor(handle.bridge_ptr()) }
    }

    fn assert_not_registered(handle: &InterfaceHandle<Self, RustRenderInterface>) {
        let current_interface_ptr = core::get_render_interface();

        let self_ptr = unsafe { handle.bridge_ptr() }.cast();

        assert_ne!(
            current_interface_ptr, self_ptr,
            "OwnedInterface dropped while still registered as RmlUI's render interface"
        );
    }
}

// Implemented on a shared borrow so the value and C++ object outlives the pointer.
impl<T: RenderInterface> IntoRawInterface<RmlRenderInterface>
    for &OwnedInterface<T, RustRenderInterface>
{
    fn into_raw(self) -> RawInterface<RmlRenderInterface> {
        // `RustRenderInterface` is a subclass of `RmlRenderInterface` so the cast is valid.
        RawInterface::new(self.as_sys_ptr().cast())
    }
}
