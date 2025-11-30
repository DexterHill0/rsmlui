#pragma once
#include <RmlUi/Core/SystemInterface.h>

#include <memory>

#include "src/ffi/system_interface.rs.h"

namespace rsmlui {
namespace system_interface {
    struct RustSystemInterface: public Rml::SystemInterface {
        RustSystemInterface(RustInterfaceOpaque* obj) : rust_interface(obj) {}

        ~RustSystemInterface() {
            ext_drop_interface(rust_interface);
        }

        auto GetElapsedTime() -> double override {
            return ext_get_elapsed_time(rust_interface);
        }

        // auto TranslateString(Rml::String& out, const Rml::String& input)
        //     -> int override {
        //     return rsmlui_sys_rust_translate_string(rust_interface, out, input);
        // }

      private:
        RustInterfaceOpaque* rust_interface;
    };

    inline auto rust_system_interface_new(RustInterfaceOpaque* obj)
        -> std::unique_ptr<RustSystemInterface> {
        return std::make_unique<RustSystemInterface>(obj);
    }

} // namespace system_interface

} // namespace rsmlui
