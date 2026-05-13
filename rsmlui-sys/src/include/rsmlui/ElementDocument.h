#pragma once
#include <RmlUi/Core/ElementDocument.h>

#include "rust/cxx.h"

namespace rsmlui {
inline void element_document_show(
    Rml::ElementDocument* document,
    Rml::ModalFlag modal_flag,
    Rml::FocusFlag focus_flag,
    Rml::ScrollFlag scroll_flag
) {
    document->Show(modal_flag, focus_flag, scroll_flag);
}

} // namespace rsmlui