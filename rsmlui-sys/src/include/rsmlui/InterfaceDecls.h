#pragma once
#include <RmlUi/Core/SystemInterface.h>

#include <cstddef>

#include "./Utils.h"
#include "RmlUi/Core/ElementInstancer.h"
#include "RmlUi/Core/FileInterface.h"
#include "RmlUi/Core/Plugin.h"
#include "RmlUi/Core/RenderInterface.h"
#include "src/ffi/interfaces/bridge.rs.h"

namespace rsmlui::system_interface {

struct RustSystemInterface:
    public Rml::SystemInterface,
    public interfaces::InterfaceBridge {
    RustSystemInterface(const void* rust_meta, void* rust_data) :
        interfaces::InterfaceBridge(rust_meta, rust_data) {}

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
};

} // namespace rsmlui::system_interface

namespace rsmlui::render_interface {
struct RustRenderInterface:
    public Rml::RenderInterface,
    public interfaces::InterfaceBridge {
    RustRenderInterface(const void* rust_meta, void* rust_data) :
        interfaces::InterfaceBridge(rust_meta, rust_data) {}

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
};

} // namespace rsmlui::render_interface

namespace rsmlui::file_interface {

struct RustFileInterface:
    public Rml::FileInterface,
    public interfaces::InterfaceBridge {
    RustFileInterface(const void* rust_meta, void* rust_data) :
        interfaces::InterfaceBridge(rust_meta, rust_data) {}

    auto Open(const Rml::String& path) -> Rml::FileHandle override;
    void Close(Rml::FileHandle file) override;
    auto Read(void* buffer, size_t size, Rml::FileHandle file)
        -> size_t override;
    auto Seek(Rml::FileHandle file, long offset, int origin) -> bool override;
    auto Tell(Rml::FileHandle file) -> size_t override;
    auto Length(Rml::FileHandle file) -> size_t override;
    auto LoadFile(const Rml::String& path, Rml::String& out_data)
        -> bool override;
};

} // namespace rsmlui::file_interface

namespace rsmlui::plugin {

struct RustPluginInterface:
    public Rml::Plugin,
    public interfaces::InterfaceBridge {
    RustPluginInterface(const void* rust_meta, void* rust_data) :
        interfaces::InterfaceBridge(rust_meta, rust_data) {}

    auto OnInitialise() -> void override;
    auto OnShutdown() -> void override;
    auto OnDocumentOpen(Rml::Context* context, const Rml::String& document_path)
        -> void override;
    auto OnDocumentLoad(Rml::ElementDocument* document) -> void override;
    auto OnDocumentUnload(Rml::ElementDocument* document) -> void override;
    auto OnContextCreate(Rml::Context* context) -> void override;
    auto OnContextDestroy(Rml::Context* context) -> void override;
    auto OnElementCreate(Rml::Element* element) -> void override;
    auto OnElementDestroy(Rml::Element* element) -> void override;
    auto OnDataModelCreate(Rml::Context* context, const Rml::String& name)
        -> void override;
    auto OnDataModelDestroy(Rml::Context* context, const Rml::String& name)
        -> void override;
};

} // namespace rsmlui::plugin

namespace rsmlui::element_instancer {
struct RustElementInstancer:
    public Rml::ElementInstancer,
    public interfaces::InterfaceBridge {
    RustElementInstancer(const void* rust_meta, void* rust_data) :
        interfaces::InterfaceBridge(rust_meta, rust_data) {}

    auto InstanceElement(
        Rml::Element* parent,
        const Rml::String& tag,
        const Rml::XMLAttributes& attributes
    ) -> Rml::ElementPtr override;

    auto ReleaseElement(Rml::Element* element) -> void override;
};

} // namespace rsmlui::element_instancer