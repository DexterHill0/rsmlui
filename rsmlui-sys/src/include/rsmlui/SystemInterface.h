#pragma once
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/SystemInterface.h>

#include "./InterfaceDecls.h"
#include "rust/cxx.h"
#include "src/ffi/system_interface.rs.h"

namespace rsmlui::system_interface {

inline auto RustSystemInterface::GetElapsedTime() -> double {
    return rust_get_elapsed_time(this);
}

inline auto RustSystemInterface::TranslateString(
    Rml::String& translated,
    const Rml::String& input
) -> int {
    translated = rust_translate_string(this, rust::Str(input)).c_str();
    return 1;
}

inline void RustSystemInterface::JoinPath(
    Rml::String& translated_path,
    const Rml::String& document_path,
    const Rml::String& path
) {
    translated_path =
        rust_join_path(this, rust::Str(document_path), rust::Str(path)).c_str();
}

inline auto
RustSystemInterface::LogMessage(Rml::Log::Type type, const Rml::String& message)
    -> bool {
    return rust_log_message(this, type, rust::Str(message));
}

inline void
RustSystemInterface::SetMouseCursor(const Rml::String& cursor_name) {
    rust_set_mouse_cursor(this, rust::Str(cursor_name));
}

inline void RustSystemInterface::SetClipboardText(const Rml::String& text) {
    rust_set_clipboard_text(this, rust::Str(text));
}

inline void RustSystemInterface::GetClipboardText(Rml::String& text) {
    text = rust_get_clipboard_text(this).c_str();
}

inline void RustSystemInterface::ActivateKeyboard(
    Rml::Vector2f caret_position,
    float line_height
) {
    rust_activate_keyboard(this, caret_position, line_height);
}

inline void RustSystemInterface::DeactivateKeyboard() {
    rust_deactivate_keyboard(this);
}

inline auto
new_rust_system_interface(const Opaque* rust_meta, Opaque* rust_data)
    -> RustSystemInterface* {
    return new RustSystemInterface((void*)rust_meta, (void*)rust_data);
}

inline void rust_system_interface_destructor(RustSystemInterface* obj) {
    delete obj;
}

// The following functions call the base `Rml::SystemInterface` implementation directly.
// Used by `RawSystemInterface` default methods to delegate to C++ without going through the
// Rust dispatch path.
inline auto
system_interface_default_get_elapsed_time(RustSystemInterface* interface)
    -> double {
    return interface->Rml::SystemInterface::GetElapsedTime();
}

inline auto system_interface_default_translate_string(
    RustSystemInterface* interface,
    rust::Str str_input
) -> rust::String {
    Rml::String translated;

    interface->Rml::SystemInterface::TranslateString(
        translated,
        Rml::String(str_input)
    );

    return {translated};
}

inline auto system_interface_default_join_path(
    RustSystemInterface* interface,
    rust::Str str_document_path,
    rust::Str str_path
) -> rust::String {
    Rml::String joined;
    interface->Rml::SystemInterface::JoinPath(
        joined,
        Rml::String(str_document_path),
        Rml::String(str_path)
    );

    return {joined};
}

inline auto system_interface_default_log_message(
    RustSystemInterface* interface,
    Rml::Log::Type type,
    rust::Str str_message
) -> bool {
    return interface->Rml::SystemInterface::LogMessage(
        type,
        Rml::String(str_message)
    );
}

inline void system_interface_default_set_mouse_cursor(
    RustSystemInterface* interface,
    rust::Str str_cursor_name
) {
    interface->Rml::SystemInterface::SetMouseCursor(
        Rml::String(str_cursor_name)
    );
}

inline void system_interface_default_set_clipboard_text(
    RustSystemInterface* interface,
    rust::Str str_text
) {
    interface->Rml::SystemInterface::SetClipboardText(Rml::String(str_text));
}

inline auto
system_interface_default_get_clipboard_text(RustSystemInterface* interface)
    -> rust::String {
    Rml::String out;
    interface->Rml::SystemInterface::GetClipboardText(out);

    return {out};
}

inline void system_interface_default_activate_keyboard(
    RustSystemInterface* interface,
    Rml::Vector2f caret_position,
    float line_height
) {
    interface->Rml::SystemInterface::ActivateKeyboard(
        caret_position,
        line_height
    );
}

inline void
system_interface_default_deactivate_keyboard(RustSystemInterface* interface) {
    interface->Rml::SystemInterface::DeactivateKeyboard();
}

} // namespace rsmlui::system_interface
