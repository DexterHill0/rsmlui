#pragma once
#include <RmlUi/Core/ElementInstancer.h>

#include "./InterfaceDecls.h"
#include "rsmlui/Utils.h"
#include "src/ffi/element_instancer.rs.h"

namespace rsmlui::element_instancer {

inline auto RustElementInstancer::ReleaseElement(Rml::Element* element)
    -> void {
    rust_release_element(this, element);
}

inline auto RustElementInstancer::InstanceElement(
    Rml::Element* parent,
    const Rml::String& tag,
    const Rml::XMLAttributes& attributes
) -> Rml::ElementPtr {
    rust_instance_element(this, parent, {tag}, attributes);
    return {};
}

inline auto new_rust_element_instancer(
    const rsmlui::c_void* rust_meta,
    rsmlui::c_void* rust_data
) -> RustElementInstancer* {
    return new RustElementInstancer(
        reinterpret_cast<const void*>(rust_meta),
        reinterpret_cast<void*>(rust_data)
    );
}

inline void rust_element_instancer_destructor(RustElementInstancer* obj) {
    delete obj;
}

} // namespace rsmlui::element_instancer