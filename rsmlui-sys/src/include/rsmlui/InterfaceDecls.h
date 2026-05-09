#include <RmlUi/Core/SystemInterface.h>

namespace rsmlui::system_interface {

struct RustSystemInterface: public Rml::SystemInterface {
    RustSystemInterface(void* rust_meta, void* rust_data) :
        rust_meta(rust_meta),
        rust_data(rust_data) {}

    auto GetElapsedTime() -> double override;
    auto TranslateString(Rml::String& translated, const Rml::String& input)
        -> int override;
    void JoinPath(
        Rml::String& translated_path,
        const Rml::String& document_path,
        const Rml::String& path
    ) override;
    auto LogMessage(Rml::Log::Type type, const Rml::String& message)
        -> bool override;
    void SetMouseCursor(const Rml::String& cursor_name) override;
    void SetClipboardText(const Rml::String& text) override;
    void GetClipboardText(Rml::String& text) override;
    void
    ActivateKeyboard(Rml::Vector2f caret_position, float line_height) override;
    void DeactivateKeyboard() override;

  public:
    void* rust_meta;
    void* rust_data;
};

} // namespace rsmlui::system_interface