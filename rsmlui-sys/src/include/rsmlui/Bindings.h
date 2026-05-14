// Only exists for bindgen to generate bindings

#pragma once
#include <RmlUi/Core.h>

#include "./InterfaceDecls.h"

namespace Layouts {
using SystemInterfaceLayoutGuard =
    rsmlui::system_interface::RustSystemInterface;
using RenderInterfaceLayoutGuard =
    rsmlui::render_interface::RustRenderInterface;
}; // namespace Layouts

namespace Rml {
using Log_Type = Rml::Log::Type;
};
