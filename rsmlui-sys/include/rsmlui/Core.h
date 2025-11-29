#pragma once
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/Math.h>

#include <cstdint>
#include <cstdio>
#include <memory>

#include "rsmlui/Types.h"
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
inline auto create_context(rust::String name, int width, int height)
    -> Context* {
    return Rml::CreateContext(name.c_str(), {width, height});
}

inline void set_system_interface(SystemInterface* system_interface) {
    Rml::SetSystemInterface(system_interface);
}

inline void set_render_interface(RenderInterface* render_interface) {
    Rml::SetRenderInterface(render_interface);
}
} // namespace rsmlui