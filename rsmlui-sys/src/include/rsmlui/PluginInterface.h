#pragma once
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/Plugin.h>

#include "./InterfaceDecls.h"
#include "rsmlui/Utils.h"
#include "rust/cxx.h"
#include "src/ffi/plugin_interface.rs.h"

namespace rsmlui::plugin {

inline auto RustPluginInterface::OnInitialise() -> void {
    rust_on_initialize(this);
}

inline auto RustPluginInterface::OnShutdown() -> void {
    rust_on_shutdown(this);
}

inline auto RustPluginInterface::OnDocumentOpen(
    Rml::Context* context,
    const Rml::String& document_path
) -> void {
    rust_on_document_open(this, context, {document_path});
}

inline auto RustPluginInterface::OnDocumentLoad(Rml::ElementDocument* document)
    -> void {
    rust_on_document_load(this, document);
}

inline auto
RustPluginInterface::OnDocumentUnload(Rml::ElementDocument* document) -> void {
    rust_on_document_unload(this, document);
}

inline auto RustPluginInterface::OnContextCreate(Rml::Context* context)
    -> void {
    rust_on_context_create(this, context);
}

inline auto RustPluginInterface::OnContextDestroy(Rml::Context* context)
    -> void {
    rust_on_context_destroy(this, context);
}

inline auto RustPluginInterface::OnElementCreate(Rml::Element* element)
    -> void {
    rust_on_element_create(this, element);
}

inline auto RustPluginInterface::OnElementDestroy(Rml::Element* element)
    -> void {
    rust_on_element_destroy(this, element);
}

inline auto RustPluginInterface::OnDataModelCreate(
    Rml::Context* context,
    const Rml::String& name
) -> void {
    rust_on_data_model_create(this, context, {name});
}

inline auto RustPluginInterface::OnDataModelDestroy(
    Rml::Context* context,
    const Rml::String& name
) -> void {
    rust_on_data_model_destroy(this, context, {name});
}

inline auto new_rust_plugin_interface(
    const rsmlui::c_void* rust_meta,
    rsmlui::c_void* rust_data
) -> RustPluginInterface* {
    return new RustPluginInterface(
        reinterpret_cast<const void*>(rust_meta),
        reinterpret_cast<void*>(rust_data)
    );
}

inline void rust_plugin_interface_destructor(RustPluginInterface* obj) {
    delete obj;
}

} // namespace rsmlui::plugin