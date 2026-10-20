#pragma once
#include <RmlUi/Core/XMLNodeHandler.h>
#include <RmlUi/Core/XMLParser.h>

#include <format>
#include <iostream>

#include "./InterfaceDecls.h"
#include "rsmlui/Utils.h"
#include "rust/cxx.h"
#include "src/ffi/xml_node_handler.rs.h"

namespace rsmlui::xml_node_handler {

inline auto RustXmlNodeHandler::ElementStart(
    Rml::XMLParser* parser,
    const Rml::String& name,
    const Rml::XMLAttributes& attributes
) -> Rml::Element* {
    rust_element_start(this, parser, {name}, attributes);

    // FIXME: remove this and return the value from `rust_element_start` when the XmlNodeHandlerBehaviour::element_start
    // is fixed
    return nullptr;
}

inline auto
RustXmlNodeHandler::ElementEnd(Rml::XMLParser* parser, const Rml::String& name)
    -> bool {
    return rust_element_end(this, parser, {name});
}

inline auto RustXmlNodeHandler::ElementData(
    Rml::XMLParser* parser,
    const Rml::String& data,
    Rml::XMLDataType type
) -> bool {
    return rust_element_data(this, parser, {data}, type);
}

inline auto new_rust_xml_node_handler(
    const rsmlui::c_void* rust_meta,
    rsmlui::c_void* rust_data
) -> RustXmlNodeHandler* {
    return new RustXmlNodeHandler(
        reinterpret_cast<const void*>(rust_meta),
        reinterpret_cast<void*>(rust_data)
    );
}

inline void rust_xml_node_handler_destructor(RustXmlNodeHandler* obj) {
    delete obj;
}

// RmlUi takes shared ownership of node handlers, but the handler is owned by Rust
// for as long as its `XmlNodeHandler` handle lives, so the deleter is a no-op.
// `Factory::Shutdown` releases every registered handler, which happens before the
// handle can be dropped on the Rust side.
inline void
register_xml_node_handler(rust::Str tag, Rml::XMLNodeHandler* handler) {
    Rml::XMLParser::RegisterNodeHandler(
        Rml::String(tag),
        Rml::SharedPtr<Rml::XMLNodeHandler>(handler, [](Rml::XMLNodeHandler*) {
        })
    );
}

} // namespace rsmlui::xml_node_handler
