#pragma once
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/SystemInterface.h>

#include "rust/cxx.h"
#include "src/ffi/system_interface.rs.h"

namespace rsmlui::system_interface {
// custom `SystemInterface` that holds a thin ptr to a `dyn SystemInterfaceExt`
struct RustSystemInterface, public Rml::SystemInterface {
    RustSystemInterface(const RustSystemInterface&) = delete;
    RustSystemInterface(RustSystemInterface&&) = delete;
    auto operator=(const RustSystemInterface&)->RustSystemInterface& = delete;
    auto operator=(RustSystemInterface&&)->RustSystemInterface& = delete;

    RustSystemInterface(InterfaceOpaque * obj) : rust_interface(obj) {}

    auto GetElapsedTime() -> double override {
        return rust_get_elapsed_time(rust_interface);
    }

    auto TranslateString(Rml::String & translated, const Rml::String& input)
        -> int override {
        auto rust_string =
            rust_translate_string(rust_interface, rust::Str(input));

        translated = rust_string.c_str();

        return 1;
    }

    void JoinPath(
        Rml::String & translated_path,
        const Rml::String& document_path,
        const Rml::String& path
    ) override {
        auto rust_string = rust_join_path(
            rust_interface,
            rust::Str(document_path),
            rust::Str(path)
        );

        translated_path = rust_string.c_str();
    }

    auto LogMessage(Rml::Log::Type type, const Rml::String& message)
        -> bool override {
        return rust_log_message(rust_interface, type, rust::Str(message));
    }

    void SetMouseCursor(const Rml::String& cursor_name) override {
        rust_set_mouse_cursor(rust_interface, rust::Str(cursor_name));
    };

    void SetClipboardText(const Rml::String& text) override {
        rust_set_clipboard_text(rust_interface, rust::Str(text));
    };

    void GetClipboardText(Rml::String & text) override {
        auto rust_string = rust_get_clipboard_text(rust_interface);

        text = rust_string.c_str();
    };

    void ActivateKeyboard(Rml::Vector2f caret_position, float line_height)
        override {
        rust_activate_keyboard(rust_interface, caret_position, line_height);
    };

    void DeactivateKeyboard() override {
        rust_deactivate_keyboard(rust_interface);
    };

  private:
    InterfaceOpaque* rust_interface;
};

// constructs a new `RustSystemInterface`
// this is called from rust and used to allow a trait object to act like a `Rml::SystemInterface`
inline auto new_rust_system_interface(InterfaceOpaque* obj)
    -> Rml::SystemInterface* {
    return new RustSystemInterface(obj);
}

inline void rust_system_interface_destructor(Rml::SystemInterface* obj) {
    delete obj;
}

// calls the base `SystemInterface` method implementations of a given interface
// used within rust to allow the trait to have equivalient default implementations
// assumes the interface is not null
inline auto system_interface_get_elapsed_time(Rml::SystemInterface* interface)
    -> double {
    return interface->Rml::SystemInterface::GetElapsedTime();
}

inline auto system_interface_translate_string(
    Rml::SystemInterface* interface,
    rust::Str str_input
) -> rust::String {
    Rml::String translated;
    const Rml::String input(str_input);

    interface->Rml::SystemInterface::TranslateString(translated, input);

    return {translated};
}

inline auto system_interface_join_path(
    Rml::SystemInterface* interface,
    rust::Str str_document_path,
    rust::Str str_path
) -> rust::String {
    Rml::String joined;
    const Rml::String document_path(str_document_path);
    const Rml::String path(str_path);

    interface->Rml::SystemInterface::JoinPath(joined, document_path, path);

    return {joined};
}

inline auto system_interface_log_message(
    Rml::SystemInterface* interface,
    Rml::Log::Type type,
    rust::Str str_message
) -> bool {
    const Rml::String message(str_message);

    return interface->Rml::SystemInterface::LogMessage(type, message);
}

inline void system_interface_set_mouse_cursor(
    Rml::SystemInterface* interface,
    rust::Str str_cursor_name
) {
    const Rml::String cursor_name(str_cursor_name);

    interface->Rml::SystemInterface::SetMouseCursor(cursor_name);
}

inline void system_interface_set_clipboard_text(
    Rml::SystemInterface* interface,
    rust::Str str_text
) {
    const Rml::String text(str_text);

    interface->Rml::SystemInterface::SetClipboardText(text);
}

inline auto system_interface_get_clipboard_text(Rml::SystemInterface* interface)
    -> rust::String {
    Rml::String out;

    interface->Rml::SystemInterface::GetClipboardText(out);

    return {out};
}

inline void system_interface_activate_keyboard(
    Rml::SystemInterface* interface,
    Rml::Vector2f caret_position,
    float line_height
) {
    interface->Rml::SystemInterface::ActivateKeyboard(
        caret_position,
        line_height
    );
}

inline void
system_interface_deactivate_keyboard(Rml::SystemInterface* interface) {
    interface->Rml::SystemInterface::DeactivateKeyboard();
}

} // namespace rsmlui::system_interface
