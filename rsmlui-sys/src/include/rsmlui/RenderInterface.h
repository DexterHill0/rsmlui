#pragma once
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/RenderInterface.h>

#ifdef RSMLUI_RENDERER_GL2
    #include <RmlUi_Renderer_GL2.h>
#endif
#ifdef RSMLUI_RENDERER_GL3
    #include <RmlUi_Renderer_GL3.h>
#endif

#include "./InterfaceDecls.h"
#include "RmlUi/Core/Types.h"
#include "rust/cxx.h"
#include "src/ffi/render_interface.rs.h"

namespace rsmlui::render_interface {

inline auto RustRenderInterface::CompileGeometry(
    Rml::Span<const Rml::Vertex> vertices,
    Rml::Span<const int> indices
) -> Rml::CompiledGeometryHandle {
    return rust_compile_geometry(
        this,
        rust::Slice(vertices.data(), vertices.size()),
        rust::Slice(indices.data(), indices.size())
    );
}

inline void RustRenderInterface::RenderGeometry(
    Rml::CompiledGeometryHandle geometry,
    Rml::Vector2f translation,
    Rml::TextureHandle texture
) {
    rust_render_geometry(this, geometry, translation, texture);
}

inline void
RustRenderInterface::ReleaseGeometry(Rml::CompiledGeometryHandle geometry) {
    rust_release_geometry(this, geometry);
}

inline auto RustRenderInterface::LoadTexture(
    Rml::Vector2i& texture_dimensions,
    const Rml::String& source
) -> Rml::TextureHandle {
    return rust_load_texture(this, &texture_dimensions, rust::Str(source));
}

inline auto RustRenderInterface::GenerateTexture(
    Rml::Span<const Rml::byte> source,
    Rml::Vector2i source_dimensions
) -> Rml::TextureHandle {
    return rust_generate_texture(
        this,
        rust::Slice<const uint8_t>(source.data(), source.size()),
        source_dimensions
    );
}

inline void RustRenderInterface::ReleaseTexture(Rml::TextureHandle texture) {
    rust_release_texture(this, texture);
}

inline void RustRenderInterface::EnableScissorRegion(bool enable) {
    rust_enable_scissor_region(this, enable);
}

inline void RustRenderInterface::SetScissorRegion(Rml::Rectanglei region) {
    rust_set_scissor_region(this, region);
}

inline void RustRenderInterface::EnableClipMask(bool enable) {
    rust_enable_clip_mask(this, enable);
}

inline void RustRenderInterface::RenderToClipMask(
    Rml::ClipMaskOperation operation,
    Rml::CompiledGeometryHandle geometry,
    Rml::Vector2f translation
) {
    rust_render_to_clip_mask(this, operation, geometry, translation);
}

inline void RustRenderInterface::SetTransform(const Rml::Matrix4f* transform) {
    rust_set_transform(this, transform);
}

inline auto RustRenderInterface::PushLayer() -> Rml::LayerHandle {
    return rust_push_layer(this);
}

inline void RustRenderInterface::CompositeLayers(
    Rml::LayerHandle source,
    Rml::LayerHandle destination,
    Rml::BlendMode blend_mode,
    Rml::Span<const Rml::CompiledFilterHandle> filters
) {
    rust_composite_layers(
        this,
        source,
        destination,
        blend_mode,
        rust::Slice<const Rml::CompiledFilterHandle>(
            filters.data(),
            filters.size()
        )
    );
}

inline void RustRenderInterface::PopLayer() {
    rust_pop_layer(this);
}

inline auto RustRenderInterface::SaveLayerAsTexture() -> Rml::TextureHandle {
    return rust_save_layer_as_texture(this);
}

inline auto RustRenderInterface::SaveLayerAsMaskImage()
    -> Rml::CompiledFilterHandle {
    return rust_save_layer_as_mask_image(this);
}

inline auto RustRenderInterface::CompileFilter(
    const Rml::String& name,
    const Rml::Dictionary& parameters
) -> Rml::CompiledFilterHandle {
    return rust_compile_filter(this, rust::Str(name), &parameters);
}

inline void
RustRenderInterface::ReleaseFilter(Rml::CompiledFilterHandle filter) {
    rust_release_filter(this, filter);
}

inline auto RustRenderInterface::CompileShader(
    const Rml::String& name,
    const Rml::Dictionary& parameters
) -> Rml::CompiledShaderHandle {
    return rust_compile_shader(this, rust::Str(name), &parameters);
}

inline void RustRenderInterface::RenderShader(
    Rml::CompiledShaderHandle shader,
    Rml::CompiledGeometryHandle geometry,
    Rml::Vector2f translation,
    Rml::TextureHandle texture
) {
    rust_render_shader(this, shader, geometry, translation, texture);
}

inline void
RustRenderInterface::ReleaseShader(Rml::CompiledShaderHandle shader) {
    rust_release_shader(this, shader);
}

inline auto new_rust_render_interface(
    const rsmlui::Opaque* rust_meta,
    rsmlui::Opaque* rust_data
) -> RustRenderInterface* {
    return new RustRenderInterface((void*)rust_meta, (void*)rust_data);
}

inline void rust_render_interface_destructor(RustRenderInterface* obj) {
    delete obj;
}

// The following functions call the base `Rml::RenderInterface` implementation directly.
// Used by the default method implementations on the safe crate's `RenderInterface` trait.
inline void render_interface_default_enable_clip_mask(
    RustRenderInterface* interface,
    bool enable
) {
    interface->Rml::RenderInterface::EnableClipMask(enable);
}

inline void render_interface_default_render_to_clip_mask(
    RustRenderInterface* interface,
    Rml::ClipMaskOperation operation,
    Rml::CompiledGeometryHandle geometry,
    Rml::Vector2f translation
) {
    interface->Rml::RenderInterface::RenderToClipMask(
        operation,
        geometry,
        translation
    );
}

inline void render_interface_default_set_transform(
    RustRenderInterface* interface,
    const Rml::Matrix4f* transform
) {
    interface->Rml::RenderInterface::SetTransform(transform);
}

inline auto render_interface_default_push_layer(RustRenderInterface* interface)
    -> Rml::LayerHandle {
    return interface->Rml::RenderInterface::PushLayer();
}

inline void render_interface_default_composite_layers(
    RustRenderInterface* interface,
    Rml::LayerHandle source,
    Rml::LayerHandle destination,
    Rml::BlendMode blend_mode,
    rust::Slice<const Rml::CompiledFilterHandle> filters
) {
    interface->Rml::RenderInterface::CompositeLayers(
        source,
        destination,
        blend_mode,
        Rml::Span(filters.data(), filters.size())
    );
}

inline void render_interface_default_pop_layer(RustRenderInterface* interface) {
    interface->Rml::RenderInterface::PopLayer();
}

inline auto
render_interface_default_save_layer_as_texture(RustRenderInterface* interface)
    -> Rml::TextureHandle {
    return interface->Rml::RenderInterface::SaveLayerAsTexture();
}

inline auto render_interface_default_save_layer_as_mask_image(
    RustRenderInterface* interface
) -> Rml::CompiledFilterHandle {
    return interface->Rml::RenderInterface::SaveLayerAsMaskImage();
}

inline auto render_interface_default_compile_filter(
    RustRenderInterface* interface,
    rust::Str name,
    const Rml::Dictionary* parameters
) -> Rml::CompiledFilterHandle {
    return interface->Rml::RenderInterface::CompileFilter(
        Rml::String(name),
        *parameters
    );
}

inline void render_interface_default_release_filter(
    RustRenderInterface* interface,
    Rml::CompiledFilterHandle filter
) {
    interface->Rml::RenderInterface::ReleaseFilter(filter);
}

inline auto render_interface_default_compile_shader(
    RustRenderInterface* interface,
    rust::Str name,
    const Rml::Dictionary* parameters
) -> Rml::CompiledShaderHandle {
    return interface->Rml::RenderInterface::CompileShader(
        Rml::String(name),
        *parameters
    );
}

inline void render_interface_default_render_shader(
    RustRenderInterface* interface,
    Rml::CompiledShaderHandle shader,
    Rml::CompiledGeometryHandle geometry,
    Rml::Vector2f translation,
    Rml::TextureHandle texture
) {
    interface->Rml::RenderInterface::RenderShader(
        shader,
        geometry,
        translation,
        texture
    );
}

inline void render_interface_default_release_shader(
    RustRenderInterface* interface,
    Rml::CompiledShaderHandle shader
) {
    interface->Rml::RenderInterface::ReleaseShader(shader);
}

} // namespace rsmlui::render_interface

#ifdef RSMLUI_RENDERER_GL2
namespace rsmlui::render_interface {
inline auto new_gl2_render_interface() -> Rml::RenderInterface* {
    return new RenderInterface_GL2();
}

inline void gl2_render_interface_destructor(Rml::RenderInterface* interface) {
    delete interface;
}
} // namespace rsmlui::render_interface
#endif

#ifdef RSMLUI_RENDERER_GL3
namespace rsmlui::render_interface {
inline auto new_gl3_render_interface() -> Rml::RenderInterface* {
    return new RenderInterface_GL3();
}

inline void gl3_render_interface_destructor(Rml::RenderInterface* interface) {
    delete interface;
}
} // namespace rsmlui::render_interface
#endif