#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    unsafe extern "C++" {
        #[cxx_name = "SystemInterface"]
        type RmlSystemInterface = crate::ffi::system_interface::RmlSystemInterface;
        #[cxx_name = "RenderInterface"]
        type RmlRenderInterface = crate::ffi::render_interface::RmlRenderInterface;

        type Context = crate::context::Context;

        type Vector2i = crate::Rml_Vector2i;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Core.h");

        fn get_version() -> String;

        fn initialise() -> bool;
        fn shutdown();

        fn load_font_face(path: String) -> bool;

        fn create_context(name: String, dimensions: Vector2i) -> *mut Context;

        unsafe fn set_system_interface(system_interface: *mut RmlSystemInterface);
        unsafe fn set_render_interface(render_interface: *mut RmlRenderInterface);
    }
}

// /// Sets the [`SystemInterface`](https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/system.html) to a custom trait-based interface.
// ///
// // pub unsafe fn set_system_interface<I>(system_interface: I)
// // where
// //     I: InterfaceInstancer,
// //     InterfaceState<I>: SystemInterfaceBehaviour,
// // {
// //     unsafe { ffi::set_system_interface(system_interface.into().raw) }
// // }

// pub unsafe fn set_system_interface<I>(system_interface: &I)
// where
//     I: AsInterfacePtr<Interface = RmlSystemInterface>,
// {
//     unsafe { ffi::set_system_interface(system_interface.as_ptr()) }
// }

// /// Sets the [`RenderInterface`](https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/render.html) to a custom trait-based interface.
// ///
// pub unsafe fn set_render_interface<I: Into<RenderInterface>>(render_interface: I) {
//     unsafe { ffi::set_render_interface(render_interface.into().raw) }
// }

pub use ffi::{
    create_context, get_version, initialise, load_font_face, set_render_interface,
    set_system_interface, shutdown,
};
