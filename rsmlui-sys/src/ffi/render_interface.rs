use std::mem::offset_of;

use crate::interfaces::InterfaceBridgeLayout;
use crate::utils::fat_from_cpp;
use crate::{
    Layouts_RenderInterfaceLayoutGuard, Rml_CompiledFilterHandle, Rml_CompiledGeometryHandle,
    Rml_CompiledShaderHandle, Rml_LayerHandle, Rml_TextureHandle, const_assert_eq,
};

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
    ) -> Rml_CompiledGeometryHandle;
    unsafe fn render_geometry(
        &mut self,
        geometry: Rml_CompiledGeometryHandle,
        translation: Vector2f,
        texture: Rml_TextureHandle,
    );
    unsafe fn release_geometry(&mut self, geometry: Rml_CompiledGeometryHandle);
    unsafe fn load_texture(
        &mut self,
        texture_dimensions: *mut Vector2i,
        source: &str,
    ) -> Rml_TextureHandle;
    unsafe fn generate_texture(
        &mut self,
        source: &[u8],
        source_dimensions: Vector2i,
    ) -> Rml_TextureHandle;
    unsafe fn release_texture(&mut self, texture: Rml_TextureHandle);
    unsafe fn enable_scissor_region(&mut self, enable: bool);
    unsafe fn set_scissor_region(&mut self, region: Rectanglei);
    unsafe fn enable_clip_mask(&mut self, enable: bool);
    unsafe fn render_to_clip_mask(
        &mut self,
        operation: ClipMaskOperation,
        geometry: Rml_CompiledGeometryHandle,
        translation: Vector2f,
    );
    unsafe fn set_transform(&mut self, transform: *const Matrix4f);
    unsafe fn push_layer(&mut self) -> Rml_LayerHandle;
    unsafe fn composite_layers(
        &mut self,
        source: Rml_LayerHandle,
        destination: Rml_LayerHandle,
        blend_mode: BlendMode,
        filters: &[Rml_CompiledFilterHandle],
    );
    unsafe fn pop_layer(&mut self);
    unsafe fn save_layer_as_texture(&mut self) -> Rml_TextureHandle;
    unsafe fn save_layer_as_mask_image(&mut self) -> Rml_CompiledFilterHandle;
    unsafe fn compile_filter(
        &mut self,
        name: &str,
        parameters: *const Dictionary,
    ) -> Rml_CompiledFilterHandle;
    unsafe fn release_filter(&mut self, filter: Rml_CompiledFilterHandle);
    unsafe fn compile_shader(
        &mut self,
        name: &str,
        parameters: *const Dictionary,
    ) -> Rml_CompiledShaderHandle;
    unsafe fn render_shader(
        &mut self,
        shader: Rml_CompiledShaderHandle,
        geometry: Rml_CompiledGeometryHandle,
        translation: Vector2f,
        texture: Rml_TextureHandle,
    );
    unsafe fn release_shader(&mut self, shader: Rml_CompiledShaderHandle);
}

unsafe fn rust_compile_geometry(
    cpp_this: *mut RustRenderInterface,
    vertices: &[Vertex],
    indices: &[i32],
) -> Rml_CompiledGeometryHandle {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this))
            .compile_geometry(vertices, indices)
    }
}

unsafe fn rust_render_geometry(
    cpp_this: *mut RustRenderInterface,
    geometry: Rml_CompiledGeometryHandle,
    translation: Vector2f,
    texture: Rml_TextureHandle,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).render_geometry(
            geometry,
            translation,
            texture,
        )
    }
}

unsafe fn rust_release_geometry(
    cpp_this: *mut RustRenderInterface,
    geometry: Rml_CompiledGeometryHandle,
) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).release_geometry(geometry) }
}

unsafe fn rust_load_texture(
    cpp_this: *mut RustRenderInterface,
    texture_dimensions: *mut Vector2i,
    source: &str,
) -> Rml_TextureHandle {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this))
            .load_texture(texture_dimensions, source)
    }
}

unsafe fn rust_generate_texture(
    cpp_this: *mut RustRenderInterface,
    source: &[u8],
    source_dimensions: Vector2i,
) -> Rml_TextureHandle {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this))
            .generate_texture(source, source_dimensions)
    }
}

unsafe fn rust_release_texture(cpp_this: *mut RustRenderInterface, texture: Rml_TextureHandle) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).release_texture(texture) }
}

unsafe fn rust_enable_scissor_region(cpp_this: *mut RustRenderInterface, enable: bool) {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).enable_scissor_region(enable)
    }
}

unsafe fn rust_set_scissor_region(cpp_this: *mut RustRenderInterface, region: Rectanglei) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).set_scissor_region(region) }
}

unsafe fn rust_enable_clip_mask(cpp_this: *mut RustRenderInterface, enable: bool) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).enable_clip_mask(enable) }
}

unsafe fn rust_render_to_clip_mask(
    cpp_this: *mut RustRenderInterface,
    operation: ClipMaskOperation,
    geometry: Rml_CompiledGeometryHandle,
    translation: Vector2f,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).render_to_clip_mask(
            operation,
            geometry,
            translation,
        )
    }
}

unsafe fn rust_set_transform(cpp_this: *mut RustRenderInterface, transform: *const Matrix4f) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).set_transform(transform) }
}

unsafe fn rust_push_layer(cpp_this: *mut RustRenderInterface) -> Rml_LayerHandle {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).push_layer() }
}

unsafe fn rust_composite_layers(
    cpp_this: *mut RustRenderInterface,
    source: Rml_LayerHandle,
    destination: Rml_LayerHandle,
    blend_mode: BlendMode,
    filters: &[Rml_CompiledFilterHandle],
) {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).composite_layers(
            source,
            destination,
            blend_mode,
            filters,
        )
    }
}

unsafe fn rust_pop_layer(cpp_this: *mut RustRenderInterface) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).pop_layer() }
}

unsafe fn rust_save_layer_as_texture(cpp_this: *mut RustRenderInterface) -> Rml_TextureHandle {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).save_layer_as_texture() }
}

unsafe fn rust_save_layer_as_mask_image(
    cpp_this: *mut RustRenderInterface,
) -> Rml_CompiledFilterHandle {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).save_layer_as_mask_image() }
}

unsafe fn rust_compile_filter(
    cpp_this: *mut RustRenderInterface,
    name: &str,
    parameters: *const Dictionary,
) -> Rml_CompiledFilterHandle {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).compile_filter(name, parameters)
    }
}

unsafe fn rust_release_filter(
    cpp_this: *mut RustRenderInterface,
    filter: Rml_CompiledFilterHandle,
) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).release_filter(filter) }
}

unsafe fn rust_compile_shader(
    cpp_this: *mut RustRenderInterface,
    name: &str,
    parameters: *const Dictionary,
) -> Rml_CompiledShaderHandle {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).compile_shader(name, parameters)
    }
}

unsafe fn rust_render_shader(
    cpp_this: *mut RustRenderInterface,
    shader: Rml_CompiledShaderHandle,
    geometry: Rml_CompiledGeometryHandle,
    translation: Vector2f,
    texture: Rml_TextureHandle,
) {
    unsafe {
        (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).render_shader(
            shader,
            geometry,
            translation,
            texture,
        )
    }
}

unsafe fn rust_release_shader(
    cpp_this: *mut RustRenderInterface,
    shader: Rml_CompiledShaderHandle,
) {
    unsafe { (*fat_from_cpp::<_, dyn RenderInterfaceBridge>(cpp_this)).release_shader(shader) }
}

#[cxx::bridge]
mod ffi {
    #[namespace = "Rml"]
    extern "C++" {
        #[cxx_name = "RenderInterface"]
        type RmlRenderInterface;

        type Vertex = crate::Rml_Vertex;
        type Vector2f = crate::Rml_Vector2f;
        type Vector2i = crate::Rml_Vector2i;
        type Rectanglei = crate::Rml_Rectanglei;
        type ClipMaskOperation = crate::Rml_ClipMaskOperation;
        type BlendMode = crate::Rml_BlendMode;
        type Matrix4f = crate::Rml_Matrix4f;
        type Dictionary = crate::dictionary::Dictionary;
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
        unsafe fn render_interface_default_enable_clip_mask(
            ptr: *mut RustRenderInterface,
            enable: bool,
        );
        #[doc(hidden)]
        unsafe fn render_interface_default_render_to_clip_mask(
            ptr: *mut RustRenderInterface,
            operation: ClipMaskOperation,
            geometry: usize, // `Rml_CompiledGeometryHandle`
            translation: Vector2f,
        );
        #[doc(hidden)]
        unsafe fn render_interface_default_set_transform(
            ptr: *mut RustRenderInterface,
            transform: *const Matrix4f,
        );
        #[doc(hidden)]
        unsafe fn render_interface_default_push_layer(ptr: *mut RustRenderInterface) -> usize; // `Rml_LayerHandle`
        #[doc(hidden)]
        unsafe fn render_interface_default_composite_layers(
            ptr: *mut RustRenderInterface,
            source: usize,      // `Rml_LayerHandle`
            destination: usize, // `Rml_LayerHandle`
            blend_mode: BlendMode,
            filters: &[usize], // `&[Rml_CompiledFilterHandle]`
        );
        #[doc(hidden)]
        unsafe fn render_interface_default_pop_layer(ptr: *mut RustRenderInterface);
        #[doc(hidden)]
        unsafe fn render_interface_default_save_layer_as_texture(
            ptr: *mut RustRenderInterface,
        ) -> usize; // `Rml_TextureHandle`
        #[doc(hidden)]
        unsafe fn render_interface_default_save_layer_as_mask_image(
            ptr: *mut RustRenderInterface,
        ) -> usize; // `Rml_CompiledFilterHandle`
        #[doc(hidden)]
        unsafe fn render_interface_default_compile_filter(
            ptr: *mut RustRenderInterface,
            name: &str,
            parameters: *const Dictionary,
        ) -> usize; // `Rml_CompiledFilterHandle`
        #[doc(hidden)]
        unsafe fn render_interface_default_release_filter(
            ptr: *mut RustRenderInterface,
            filter: usize, // `Rml_CompiledFilterHandle`
        );
        #[doc(hidden)]
        unsafe fn render_interface_default_compile_shader(
            ptr: *mut RustRenderInterface,
            name: &str,
            parameters: *const Dictionary,
        ) -> usize; // `Rml_CompiledShaderHandle`
        #[doc(hidden)]
        unsafe fn render_interface_default_render_shader(
            ptr: *mut RustRenderInterface,
            shader: usize,   // `Rml_CompiledShaderHandle`
            geometry: usize, // `Rml_CompiledGeometryHandle`
            translation: Vector2f,
            texture: usize, // `Rml_TextureHandle`
        );
        #[doc(hidden)]
        unsafe fn render_interface_default_release_shader(
            ptr: *mut RustRenderInterface,
            shader: usize, // `Rml_CompiledShaderHandle`
        );

        #[cfg(feature = "renderer-gl2")]
        fn new_gl2_render_interface() -> *mut RmlRenderInterface;
        #[cfg(feature = "renderer-gl2")]
        unsafe fn gl2_render_interface_destructor(interface: *mut RmlRenderInterface);
    }

    extern "Rust" {
        unsafe fn rust_compile_geometry(
            cpp_this: *mut RustRenderInterface,
            vertices: &[Vertex],
            indices: &[i32],
        ) -> usize;
        unsafe fn rust_render_geometry(
            cpp_this: *mut RustRenderInterface,
            geometry: usize,
            translation: Vector2f,
            texture: usize,
        );
        unsafe fn rust_release_geometry(cpp_this: *mut RustRenderInterface, geometry: usize);
        unsafe fn rust_load_texture(
            cpp_this: *mut RustRenderInterface,
            texture_dimensions: *mut Vector2i,
            source: &str,
        ) -> usize;
        unsafe fn rust_generate_texture(
            cpp_this: *mut RustRenderInterface,
            source: &[u8],
            source_dimensions: Vector2i,
        ) -> usize;
        unsafe fn rust_release_texture(cpp_this: *mut RustRenderInterface, texture: usize);
        unsafe fn rust_enable_scissor_region(cpp_this: *mut RustRenderInterface, enable: bool);
        unsafe fn rust_set_scissor_region(cpp_this: *mut RustRenderInterface, region: Rectanglei);
        unsafe fn rust_enable_clip_mask(cpp_this: *mut RustRenderInterface, enable: bool);
        unsafe fn rust_render_to_clip_mask(
            cpp_this: *mut RustRenderInterface,
            operation: ClipMaskOperation,
            geometry: usize,
            translation: Vector2f,
        );
        unsafe fn rust_set_transform(
            cpp_this: *mut RustRenderInterface,
            transform: *const Matrix4f,
        );
        unsafe fn rust_push_layer(cpp_this: *mut RustRenderInterface) -> usize;
        unsafe fn rust_composite_layers(
            cpp_this: *mut RustRenderInterface,
            source: usize,
            destination: usize,
            blend_mode: BlendMode,
            filters: &[usize],
        );
        unsafe fn rust_pop_layer(cpp_this: *mut RustRenderInterface);
        unsafe fn rust_save_layer_as_texture(cpp_this: *mut RustRenderInterface) -> usize;
        unsafe fn rust_save_layer_as_mask_image(cpp_this: *mut RustRenderInterface) -> usize;
        unsafe fn rust_compile_filter(
            cpp_this: *mut RustRenderInterface,
            name: &str,
            parameters: *const Dictionary,
        ) -> usize;
        unsafe fn rust_release_filter(cpp_this: *mut RustRenderInterface, filter: usize);
        unsafe fn rust_compile_shader(
            cpp_this: *mut RustRenderInterface,
            name: &str,
            parameters: *const Dictionary,
        ) -> usize;
        unsafe fn rust_render_shader(
            cpp_this: *mut RustRenderInterface,
            shader: usize,
            geometry: usize,
            translation: Vector2f,
            texture: usize,
        );
        unsafe fn rust_release_shader(cpp_this: *mut RustRenderInterface, shader: usize);
    }
}

pub use ffi::{
    BlendMode, ClipMaskOperation, Dictionary, Matrix4f, Rectanglei, RmlRenderInterface,
    RustRenderInterface, Vector2f, Vector2i, Vertex, new_rust_render_interface,
    render_interface_default_compile_filter, render_interface_default_compile_shader,
    render_interface_default_composite_layers, render_interface_default_enable_clip_mask,
    render_interface_default_pop_layer, render_interface_default_push_layer,
    render_interface_default_release_filter, render_interface_default_release_shader,
    render_interface_default_render_shader, render_interface_default_render_to_clip_mask,
    render_interface_default_save_layer_as_mask_image,
    render_interface_default_save_layer_as_texture, render_interface_default_set_transform,
    rust_render_interface_destructor,
};
#[cfg(feature = "renderer-gl2")]
pub use ffi::{gl2_render_interface_destructor, new_gl2_render_interface};
