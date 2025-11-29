#pragma once
#include <RmlUi_Backend.h>

#include "rust/cxx.h"

namespace rsmlui {
namespace backend {
    inline auto initialize(
        rust::String window_name,
        int32_t width,
        int32_t height,
        bool allow_resize
    ) -> bool {
        return Backend::Initialize(
            window_name.c_str(),
            width,
            height,
            allow_resize
        );
    }

    inline void shutdown() {
        Backend::Shutdown();
    }

    inline auto get_system_interface() -> Rml::SystemInterface* {
        return Backend::GetSystemInterface();
    }

    inline auto get_render_interface() -> Rml::RenderInterface* {
        return Backend::GetRenderInterface();
    }

    inline auto process_events(
        Rml::Context* context,
        rust::Fn<bool(
            Rml::Context* ctx,
            Rml::Input::KeyIdentifier key,
            int32_t key_modifier,
            float native_dp_ratio,
            bool priority
        )> rust_callback,
        bool power_save
    ) -> bool {
        static auto stored_rust_callback = rust_callback;

        static bool (*callback)(
            Rml::Context*,
            Rml::Input::KeyIdentifier,
            int,
            float,
            bool
        ) = [](Rml::Context* ctx,
               Rml::Input::KeyIdentifier key,
               int key_modifier,
               float native_dp_ratio,
               bool priority) -> bool {
            return stored_rust_callback(
                ctx,
                key,
                key_modifier,
                native_dp_ratio,
                priority
            );
        };

        return Backend::ProcessEvents(context, callback, power_save);
    }

    inline void request_exit() {
        Backend::RequestExit();
    }

    inline void begin_frame() {
        Backend::BeginFrame();
    }

    inline void present_frame() {
        Backend::PresentFrame();
    }
} // namespace backend

} // namespace rsmlui
