#pragma once
#include <RmlUi/Core/SystemInterface.h>

#include "RmlUi/Core/FileInterface.h"
#include "RmlUi/Core/RenderInterface.h"

namespace rsmlui {
class Opaque;
} // namespace rsmlui

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

namespace rsmlui::render_interface {
struct RustRenderInterface: public Rml::RenderInterface {
    RustRenderInterface(void* rust_meta, void* rust_data) :
        rust_meta(rust_meta),
        rust_data(rust_data) {}

    auto CompileGeometry(
        Rml::Span<const Rml::Vertex> vertices,
        Rml::Span<const int> indices
    ) -> Rml::CompiledGeometryHandle override;
    void RenderGeometry(
        Rml::CompiledGeometryHandle geometry,
        Rml::Vector2f translation,
        Rml::TextureHandle texture
    ) override;
    void ReleaseGeometry(Rml::CompiledGeometryHandle geometry) override;
    auto
    LoadTexture(Rml::Vector2i& texture_dimensions, const Rml::String& source)
        -> Rml::TextureHandle override;
    auto GenerateTexture(
        Rml::Span<const Rml::byte> source,
        Rml::Vector2i source_dimensions
    ) -> Rml::TextureHandle override;
    void ReleaseTexture(Rml::TextureHandle texture) override;
    void EnableScissorRegion(bool enable) override;
    void SetScissorRegion(Rml::Rectanglei region) override;
    void EnableClipMask(bool enable) override;
    void RenderToClipMask(
        Rml::ClipMaskOperation operation,
        Rml::CompiledGeometryHandle geometry,
        Rml::Vector2f translation
    ) override;
    void SetTransform(const Rml::Matrix4f* transform) override;
    auto PushLayer() -> Rml::LayerHandle override;
    void CompositeLayers(
        Rml::LayerHandle source,
        Rml::LayerHandle destination,
        Rml::BlendMode blend_mode,
        Rml::Span<const Rml::CompiledFilterHandle> filters
    ) override;
    void PopLayer() override;
    auto SaveLayerAsTexture() -> Rml::TextureHandle override;
    auto SaveLayerAsMaskImage() -> Rml::CompiledFilterHandle override;
    auto
    CompileFilter(const Rml::String& name, const Rml::Dictionary& parameters)
        -> Rml::CompiledFilterHandle override;
    void ReleaseFilter(Rml::CompiledFilterHandle filter) override;
    auto
    CompileShader(const Rml::String& name, const Rml::Dictionary& parameters)
        -> Rml::CompiledShaderHandle override;
    void RenderShader(
        Rml::CompiledShaderHandle shader,
        Rml::CompiledGeometryHandle geometry,
        Rml::Vector2f translation,
        Rml::TextureHandle texture
    ) override;
    void ReleaseShader(Rml::CompiledShaderHandle shader) override;

  public:
    void* rust_meta;
    void* rust_data;
};
} // namespace rsmlui::render_interface

namespace rsmlui::file_interface {

struct RustFileInterface: public Rml::FileInterface {
    RustFileInterface(void* rust_meta, void* rust_data) :
        rust_meta(rust_meta),
        rust_data(rust_data) {}

    auto Open(const Rml::String& path) -> Rml::FileHandle override;
    void Close(Rml::FileHandle file) override;
    auto Read(void* buffer, size_t size, Rml::FileHandle file)
        -> size_t override;
    auto Seek(Rml::FileHandle file, long offset, int origin) -> bool override;
    auto Tell(Rml::FileHandle file) -> size_t override;
    auto Length(Rml::FileHandle file) -> size_t override;
    auto LoadFile(const Rml::String& path, Rml::String& out_data)
        -> bool override;

  public:
    void* rust_meta;
    void* rust_data;
};

} // namespace rsmlui::file_interface