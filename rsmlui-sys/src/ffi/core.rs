use crate::render_interface::RenderInterfaceExt;
use crate::system_interface::SystemInterfaceExt;
use crate::utils::IntoPtr;

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    unsafe extern "C++" {
        type SystemInterface = crate::ffi::system_interface::SystemInterface;
        type RenderInterface = crate::ffi::render_interface::RenderInterface;

        type Context = crate::context::Context;

        type Vector2i = crate::Rml_Vector2i;
    }

    #[namespace = "rsmlui"]
    unsafe extern "C++" {
        include!("rsmlui/Core.h");

        fn get_version() -> String;

        fn initialise() -> bool;
        fn shutdown();

        fn create_context(name: String, dimensions: Vector2i) -> *mut Context;

        pub(self) unsafe fn set_system_interface(system_interface: *mut SystemInterface);
        pub(self) unsafe fn set_render_interface(render_interface: *mut RenderInterface);
    }
}

pub unsafe fn set_system_interface_ext<I>(system_interface: I)
where
    I: SystemInterfaceExt,
    I: IntoPtr<SystemInterface>,
{
    let ptr = system_interface.into_ptr();

    dbg!(&ptr);

    unsafe { ffi::set_system_interface(ptr) }
}

pub unsafe fn set_render_interface_ext<R>(render_interface: R)
where
    R: RenderInterfaceExt,
    R: IntoPtr<RenderInterface>,
{
    unsafe { ffi::set_render_interface(render_interface.into_ptr()) }
}

pub use ffi::*;
