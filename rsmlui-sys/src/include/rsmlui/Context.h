#pragma once
#include <RmlUi/Core/Context.h>

#include <cstdint>
#include <string>

#include "RmlUi/Core/Input.h"
#include "rust/cxx.h"

namespace rsmlui {

inline auto context_update(Rml::Context* ctx) -> bool {
    return ctx->Update();
}

inline auto context_render(Rml::Context* ctx) -> bool {
    return ctx->Render();
}

inline auto context_load_document(Rml::Context* ctx, rust::String document_path)
    -> Rml::ElementDocument* {
    return ctx->LoadDocument(document_path.c_str());
}

inline auto context_set_dimensions(Rml::Context* ctx, Rml::Vector2i dimensions)
    -> void {
    ctx->SetDimensions(dimensions);
}

inline auto context_process_mouse_move(
    Rml::Context* ctx,
    int x,
    int y,
    Rml::Input::KeyModifier key_modifier_state
) -> bool {
    return ctx->ProcessMouseMove(x, y, key_modifier_state);
}

inline auto context_process_mouse_button_down(
    Rml::Context* ctx,
    int button_index,
    Rml::Input::KeyModifier key_modifier_state
) -> bool {
    return ctx->ProcessMouseButtonDown(button_index, key_modifier_state);
}

inline auto context_process_mouse_button_up(
    Rml::Context* ctx,
    int button_index,
    Rml::Input::KeyModifier key_modifier_state
) -> bool {
    return ctx->ProcessMouseButtonUp(button_index, key_modifier_state);
}

inline auto context_process_mouse_wheel(
    Rml::Context* ctx,
    Rml::Vector2f wheel_delta,
    Rml::Input::KeyModifier key_modifier_state
) -> bool {
    return ctx->ProcessMouseWheel(wheel_delta, key_modifier_state);
}

inline auto context_process_mouse_leave(Rml::Context* ctx) -> bool {
    return ctx->ProcessMouseLeave();
}

inline auto context_process_key_down(
    Rml::Context* ctx,
    Rml::Input::KeyIdentifier key_identifier,
    Rml::Input::KeyModifier key_modifier_state
) -> bool {
    return ctx->ProcessKeyDown(key_identifier, key_modifier_state);
}

inline auto context_process_key_up(
    Rml::Context* ctx,
    Rml::Input::KeyIdentifier key_identifier,
    Rml::Input::KeyModifier key_modifier_state
) -> bool {
    return ctx->ProcessKeyUp(key_identifier, key_modifier_state);
}

inline auto context_process_text_input_char(Rml::Context* ctx, int8_t character)
    -> bool {
    return ctx->ProcessTextInput(character);
}

inline auto
context_process_text_input_string(Rml::Context* ctx, const rust::String& string)
    -> bool {
    std::string std_string = {string};
    return ctx->ProcessTextInput(std_string);
}

} // namespace rsmlui