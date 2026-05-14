#pragma once
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/RenderInterface.h>

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
inline auto render_interface_default_compile_geometry(
    RustRenderInterface* interface,
    rust::Slice<const Rml::Vertex> vertices,
    rust::Slice<const int> indices
) -> Rml::CompiledGeometryHandle {
    return interface->Rml::RenderInterface::CompileGeometry(
        Rml::Span(vertices.data(), vertices.size()),
        Rml::Span(indices.data(), indices.size())
    );
}

} // namespace rsmlui::render_interface
