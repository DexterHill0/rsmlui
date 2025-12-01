#pragma once
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/Math.h>

#include "Utils.h"
#include "rust/cxx.h"

namespace rsmlui {

inline auto get_version() -> rust::String {
    Rml::String ver = Rml::GetVersion();
    return {ver.c_str()};
}

inline auto initialise() -> bool {
    return Rml::Initialise();
}

// TODO: vectors
inline auto create_context(rust::String name, Rml::Vector2i dimensions)
    -> Rml::Context* {
    return Rml::CreateContext(name.c_str(), dimensions);
}

inline void set_system_interface(Rml::SystemInterface* system_interface) {
    auto* old = Rml::GetSystemInterface();

    // // as we have essentially leaked the in rust, neither rust nor rml owns the memory,
    // // so we must make sure to drop it when its being replaced
    // // if it's not a rust interface, then it's up to the user that set the pointer
    // if (auto* rust = as_rust_interface(old)) {
    //     delete rust;
    // }

    Rml::SetSystemInterface(system_interface);
}

inline void set_render_interface(Rml::RenderInterface* render_interface) {
    // auto* old = Rml::GetRenderInterface();

    // if (auto* rust = as_rust_interface(old)) {
    //     delete rust;
    // }

    Rml::SetRenderInterface(render_interface);
}

inline auto load_font_face(rust::String path) -> bool {
    return Rml::LoadFontFace(path.c_str());
}

inline void shutdown() {
    // set them to null ourselves so we run the destructor if they contain any rust interfaces
    set_system_interface(nullptr);
    set_render_interface(nullptr);

    Rml::Shutdown();
}
} // namespace rsmlui