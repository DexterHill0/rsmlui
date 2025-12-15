#pragma once
#include <RmlUi/Core/ElementDocument.h>

#include "rust/cxx.h"

namespace rsmlui {
inline void element_document_destructor(Rml::ElementDocument* document) {
    document->~ElementDocument();
}

inline void element_document_show(Rml::ElementDocument* document) {
    document->Show();
}

} // namespace rsmlui