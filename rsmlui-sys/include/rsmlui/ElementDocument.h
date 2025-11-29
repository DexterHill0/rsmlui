#pragma once
#include <RmlUi/Core/ElementDocument.h>

#include "rsmlui/Types.h"
#include "rust/cxx.h"

namespace rsmlui {
inline auto element_document_show(ElementDocument* document) {
    return document->Show();
}

} // namespace rsmlui