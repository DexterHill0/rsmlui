use cxx::{ExternType, type_id};

unsafe impl ExternType for crate::Rml_Style_FontWeight {
    type Id = type_id!("Rml::Style::FontWeight");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for crate::Rml_Style_FontStyle {
    type Id = type_id!("Rml::Style::FontStyle");
    type Kind = cxx::kind::Trivial;
}

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

    #[namespace = "Rml::Style"]
    unsafe extern "C++" {
        type FontWeight = crate::Rml_Style_FontWeight;
        type FontStyle = crate::Rml_Style_FontStyle;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Core.h");

        fn get_version() -> String;

        fn initialise() -> bool;
        fn shutdown();

        fn load_font_face_from_file(
            path: String,
            family: String,
            style: FontStyle,
            fallback_face: bool,
            weight: FontWeight,
            face_index: i32,
        ) -> bool;
        fn load_font_face_from_memory(
            data: &[u8],
            family: String,
            style: FontStyle,
            fallback_face: bool,
            weight: FontWeight,
            face_index: i32,
        ) -> bool;

        fn create_context(name: String, dimensions: Vector2i) -> *mut Context;

        unsafe fn set_system_interface(system_interface: *mut RmlSystemInterface);
        unsafe fn set_render_interface(render_interface: *mut RmlRenderInterface);

        fn get_system_interface() -> *mut RmlSystemInterface;
        fn get_render_interface() -> *mut RmlRenderInterface;
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
    FontStyle, FontWeight, create_context, get_render_interface, get_system_interface, get_version,
    initialise, load_font_face_from_file, load_font_face_from_memory, set_render_interface,
    set_system_interface, shutdown,
};
