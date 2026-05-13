#pragma once
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/Math.h>

#include "rust/cxx.h"

namespace rsmlui {

inline auto get_version() -> rust::String {
    Rml::String ver = Rml::GetVersion();
    return {ver.c_str()};
}

inline auto initialise() -> bool {
    return Rml::Initialise();
}

inline auto create_context(rust::String name, Rml::Vector2i dimensions)
    -> Rml::Context* {
    return Rml::CreateContext(name.c_str(), dimensions);
}

inline void set_system_interface(Rml::SystemInterface* system_interface) {
    Rml::SetSystemInterface(system_interface);
}

inline auto get_system_interface() -> Rml::SystemInterface* {
    return Rml::GetSystemInterface();
}

inline void set_render_interface(Rml::RenderInterface* render_interface) {
    Rml::SetRenderInterface(render_interface);
}

inline auto get_render_interface() -> Rml::RenderInterface* {
    return Rml::GetRenderInterface();
}

inline auto load_font_face_from_file(
    rust::String path,
    rust::String family,
    Rml::Style::FontStyle style,
    bool fallback_face,
    Rml::Style::FontWeight weight,
    int face_index
) -> bool {
    return Rml::LoadFontFace(
        path.c_str(),
        family.c_str(),
        style,
        weight,
        fallback_face,
        face_index
    );
}

inline auto load_font_face_from_memory(
    rust::Slice<const uint8_t> data,
    rust::String family,
    Rml::Style::FontStyle style,
    bool fallback_face,
    Rml::Style::FontWeight weight,
    int face_index
) -> bool {
    return Rml::LoadFontFace(
        Rml::Span(data.data(), data.size()),
        family.c_str(),
        style,
        weight,
        fallback_face,
        face_index
    );
}

inline void shutdown() {
    Rml::Shutdown();
}
} // namespace rsmlui