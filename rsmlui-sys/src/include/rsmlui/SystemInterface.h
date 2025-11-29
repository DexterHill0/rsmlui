#pragma once
#include <RmlUi/Core/SystemInterface.h>

#include "rust/cxx.h"

namespace rsmlui {
namespace system_interface {
    inline auto
    system_interface_get_elapsed_time(Rml::SystemInterface* system_interface)
        -> double {
        return system_interface->GetElapsedTime();
    }

    inline auto system_interface_translate_string(
        Rml::SystemInterface* system_interface,
        std::string& translated,
        const std::string& input
    ) -> int {
        return system_interface->TranslateString(translated, input);
    }

    inline void system_interface_join_path(
        Rml::SystemInterface* system_interface,
        std::string& translated_path,
        const std::string& document_path,
        const std::string& path
    ) {
        system_interface->JoinPath(translated_path, document_path, path);
    }

    inline auto system_interface_log_message(
        Rml::SystemInterface* system_interface,
        Rml::Log::Type type,
        const std::string& message
    ) -> bool {
        return system_interface->LogMessage(type, message);
    }

    inline void system_interface_set_mouse_cursor(
        Rml::SystemInterface* system_interface,
        const std::string& cursor_name
    ) {
        system_interface->SetMouseCursor(cursor_name);
    }

    inline void system_interface_set_clipboard_text(
        Rml::SystemInterface* system_interface,
        const std::string& text
    ) {
        system_interface->SetClipboardText(text);
    }

    inline void system_interface_get_clipboard_text(
        Rml::SystemInterface* system_interface,
        std::string& text
    ) {
        system_interface->GetClipboardText(text);
    }

    inline void system_interface_activate_keyboard(
        Rml::SystemInterface* system_interface,
        Rml::Vector2f caret_position,
        float line_height
    ) {
        system_interface->ActivateKeyboard(caret_position, line_height);
    }

    inline void system_interface_deactivate_keyboard(
        Rml::SystemInterface* system_interface
    ) {
        system_interface->DeactivateKeyboard();
    }
} // namespace system_interface

} // namespace rsmlui
