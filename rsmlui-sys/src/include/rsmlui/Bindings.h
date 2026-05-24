// Only exists for bindgen to generate bindings

#pragma once
#include <RmlUi/Core.h>

#include <cstdint>

#include "./InterfaceDecls.h"

namespace Layouts {
using SystemInterfaceLayoutGuard =
    rsmlui::system_interface::RustSystemInterface;
using RenderInterfaceLayoutGuard =
    rsmlui::render_interface::RustRenderInterface;
using FileInterfaceLayoutGuard = rsmlui::file_interface::RustFileInterface;
}; // namespace Layouts

namespace Rml {
using Log_Type = Rml::Log::Type;
};

namespace Misc {
static int32_t STDIO_SEEK_CUR = SEEK_CUR;
static int32_t STDIO_SEEK_END = SEEK_END;
static int32_t STDIO_SEEK_SET = SEEK_SET;
} // namespace Misc