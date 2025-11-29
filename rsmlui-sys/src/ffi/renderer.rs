use crate::bindings::Rml_Vertex;

#[cxx::bridge(namespace = "rsmlui::render_interface")]
mod ffi {
    extern "Rust" {
        type Rml_Vertex;
    }

    unsafe extern "C++" {
        include!("rsmlui/Renderer.h");

        // type RenderInterface;

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
