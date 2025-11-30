use crate::{bindings::Rml_Vertex, utils::IntoPtr};

pub trait RenderInterfaceExt {}

impl IntoPtr<RenderInterface> for *mut RenderInterface {
    fn into_ptr(self) -> *mut RenderInterface {
        self
    }
}

impl<T: RenderInterfaceExt + 'static> IntoPtr<RenderInterface> for T {
    fn into_ptr(self) -> *mut RenderInterface {
        // TODO: fix
        let boxed_trait: Box<dyn RenderInterfaceExt> = Box::new(self);

        let raw = Box::into_raw(boxed_trait) as *mut RenderInterface;

        raw

        // let unique = unsafe { rust_system_interface_new(raw) };
        // // drops rust's ownership so RmlUi can take ownership and control the lifetime of the interface
        // let raw_cpp_ptr = cxx::UniquePtr::into_raw(unique);

        // raw_cpp_ptr as *mut SystemInterface
    }
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type Rml_Vertex;
    }

    #[namespace = "Rml"]
    extern "C++" {
        type RenderInterface;
    }

    #[namespace = "rsmlui::render_interface"]
    unsafe extern "C++" {
        include!("rsmlui/Renderer.h");

        // // Opaque placeholder types
        // type Vector2f;
        // type Vector2i;
        // type Rectanglei;
        // type Matrix4f;
        // type Dictionary;

        // // Methods use Pin for mutable C++ references
        // unsafe fn render_interface_compile_geometry(
        //     ri: *mut RenderInterface,
        //     vertices: &[Rml_Vertex],
        //     indices: &[i32],
        // ) -> usize;

        // unsafe fn render_interface_render_geometry(
        //     ri: *mut RenderInterface,
        //     geometry: usize,
        //     translation: &Vector2f,
        //     texture: usize,
        // );

        // unsafe fn render_interface_release_geometry(ri: *mut RenderInterface, geometry: usize);

        // unsafe fn render_interface_load_texture(
        //     ri: *mut RenderInterface,
        //     texture_dimensions: *mut Vector2i,
        //     source: &CxxString,
        // ) -> usize;

        // unsafe fn render_interface_generate_texture(
        //     ri: *mut RenderInterface,
        //     source: &[u8],
        //     source_dimensions: &Vector2i,
        // ) -> usize;

        // unsafe fn render_interface_release_texture(ri: *mut RenderInterface, texture: usize);

        // unsafe fn render_interface_enable_scissor_region(ri: *mut RenderInterface, enable: bool);

        // unsafe fn render_interface_set_scissor_region(
        //     ri: *mut RenderInterface,
        //     region: &Rectanglei,
        // );

        // unsafe fn render_interface_enable_clip_mask(ri: *mut RenderInterface, enable: bool);

        // unsafe fn render_interface_render_to_clip_mask(
        //     ri: *mut RenderInterface,
        //     operation: i32,
        //     geometry: usize,
        //     translation: &Vector2f,
        // );

        // unsafe fn render_interface_set_transform(
        //     ri: *mut RenderInterface,
        //     transform: *const Matrix4f,
        // );

        // unsafe fn render_interface_push_layer(ri: *mut RenderInterface) -> usize;

        // unsafe fn render_interface_composite_layers(
        //     ri: *mut RenderInterface,
        //     source: usize,
        //     destination: usize,
        //     blend_mode: i32,
        //     filters: &[usize],
        // );

        // unsafe fn render_interface_pop_layer(ri: *mut RenderInterface);

        // unsafe fn render_interface_save_layer_as_texture(ri: *mut RenderInterface) -> usize;

        // unsafe fn render_interface_save_layer_as_mask_image(ri: *mut RenderInterface) -> usize;

        // unsafe fn render_interface_compile_filter(
        //     ri: *mut RenderInterface,
        //     name: &CxxString,
        //     parameters: &Dictionary,
        // ) -> usize;

        // unsafe fn render_interface_release_filter(ri: *mut RenderInterface, filter: usize);

        // unsafe fn render_interface_compile_shader(
        //     ri: *mut RenderInterface,
        //     name: &CxxString,
        //     parameters: &Dictionary,
        // ) -> usize;

        // unsafe fn render_interface_render_shader(
        //     ri: *mut RenderInterface,
        //     shader: usize,
        //     geometry: usize,
        //     translation: &Vector2f,
        //     texture: usize,
        // );

        // unsafe fn render_interface_release_shader(ri: *mut RenderInterface, shader: usize);
    }
}

pub use ffi::*;
