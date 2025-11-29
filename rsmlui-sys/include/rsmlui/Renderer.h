#pragma once
#include <RmlUi/Core/RenderInterface.h>

#include "rsmlui/Types.h"
#include "rust/cxx.h"

namespace rsmlui {
namespace render_interface {
    // inline auto render_interface_compile_geometry(
    //     RenderInterface* interface,
    //     Span<const Vertex> vertices,
    //     Span<const int> indices
    // ) -> rust::usize {
    //     return interface->CompileGeometry(vertices, indices);
    // }

    // inline void render_interface_render_geometry(
    //     RenderInterface* interface,
    //     rust::usize geometry,
    //     Vector2f translation,
    //     rust::usize texture
    // ) {
    //     interface->RenderGeometry(geometry, translation, texture);
    // }

    // inline void render_interface_release_geometry(
    //     RenderInterface* interface,
    //     rust::usize geometry
    // ) {
    //     interface->ReleaseGeometry(geometry);
    // }

    // inline auto render_interface_load_texture(
    //     RenderInterface* interface,
    //     Vector2i& texture_dimensions,
    //     rust::Str source
    // ) -> rust::usize {
    //     return interface->LoadTexture(texture_dimensions, source.data());
    // }

    // inline auto render_interface_generate_texture(
    //     RenderInterface* interface,
    //     Span<const byte> source,
    //     Vector2i source_dimensions
    // ) -> rust::usize {
    //     return interface->GenerateTexture(source, source_dimensions);
    // }

    // inline void render_interface_release_texture(
    //     RenderInterface* interface,
    //     rust::usize texture
    // ) {
    //     interface->ReleaseTexture(texture);
    // }

    // inline void render_interface_enable_scissor_region(
    //     RenderInterface* interface,
    //     bool enable
    // ) {
    //     interface->EnableScissorRegion(enable);
    // }

    // inline void render_interface_set_scissor_region(
    //     RenderInterface* interface,
    //     Rectanglei region
    // ) {
    //     interface->SetScissorRegion(region);
    // }

    // inline void
    // render_interface_enable_clip_mask(RenderInterface* interface, bool enable) {
    //     interface->EnableClipMask(enable);
    // }

    // inline void render_interface_render_to_clip_mask(
    //     RenderInterface* interface,
    //     ClipMaskOperation operation,
    //     rust::usize geometry,
    //     Vector2f translation
    // ) {
    //     interface->RenderToClipMask(operation, geometry, translation);
    // }

    // inline void render_interface_set_transform(
    //     RenderInterface* interface,
    //     const Matrix4f* transform
    // ) {
    //     interface->SetTransform(transform);
    // }

    // inline auto render_interface_push_layer(RenderInterface* interface)
    //     -> rust::usize {
    //     return interface->PushLayer();
    // }

    // inline void render_interface_composite_layers(
    //     RenderInterface* interface,
    //     rust::usize source,
    //     rust::usize destination,
    //     BlendMode blend_mode,
    //     Span<const rust::usize> filters
    // ) {
    //     interface->CompositeLayers(source, destination, blend_mode, filters);
    // }

    // inline void render_interface_pop_layer(RenderInterface* interface) {
    //     interface->PopLayer();
    // }

    // inline auto
    // render_interface_save_layer_as_texture(RenderInterface* interface)
    //     -> rust::usize {
    //     return interface->SaveLayerAsTexture();
    // }

    // inline auto
    // render_interface_save_layer_as_mask_image(RenderInterface* interface)
    //     -> rust::usize {
    //     return interface->SaveLayerAsMaskImage();
    // }

    // inline auto render_interface_compile_filter(
    //     RenderInterface* interface,
    //     const String& name,
    //     const Dictionary& parameters
    // ) -> rust::usize {
    //     return interface->CompileFilter(name, parameters);
    // }

    // inline void render_interface_release_filter(
    //     RenderInterface* interface,
    //     rust::usize filter
    // ) {
    //     interface->ReleaseFilter(filter);
    // }

    // inline auto render_interface_compile_shader(
    //     RenderInterface* interface,
    //     const String& name,
    //     const Dictionary& parameters
    // ) -> rust::usize {
    //     return interface->CompileShader(name, parameters);
    // }

    // inline void render_interface_render_shader(
    //     RenderInterface* interface,
    //     rust::usize shader,
    //     rust::usize geometry,
    //     Vector2f translation,
    //     rust::usize texture
    // ) {
    //     interface->RenderShader(shader, geometry, translation, texture);
    // }

    // inline void render_interface_release_shader(
    //     RenderInterface* interface,
    //     rust::usize shader
    // ) {
    //     interface->ReleaseShader(shader);
    // }
} // namespace render_interface

} // namespace rsmlui
