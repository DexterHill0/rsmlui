// Only exists for bindgen to generate bindings

#pragma once
#include <RmlUi/Core.h>

#include "./InterfaceDecls.h"

namespace Layouts {
using SystemInterfaceLayoutGuard =
    rsmlui::system_interface::RustSystemInterface;
};

namespace Rml {
using Log_Type = Rml::Log::Type;
};
