#pragma once

#include <RmlUi/Core/BaseXMLParser.h>
#include <RmlUi/Core/Log.h>

#include <cstddef>

namespace rsmlui {
using c_void = void;
}

namespace rsmlui::log {
using RmlLogType = Rml::Log::Type;
}

namespace rsmlui::xml {
using XmlDataType = Rml::XMLDataType;
}

namespace rsmlui::interfaces {
struct InterfaceBridge {
  protected:
    InterfaceBridge(const void* rust_meta, void* rust_data) :
        rust_meta(rust_meta),
        rust_data(rust_data) {}

  public:
    const void* rust_meta;
    void* rust_data;
};

inline auto get_meta_from_bridge(InterfaceBridge* holder) -> const void* {
    return holder->rust_meta;
}

inline auto get_data_from_bridge(InterfaceBridge* holder) -> void* {
    return holder->rust_data;
}
} // namespace rsmlui::interfaces
