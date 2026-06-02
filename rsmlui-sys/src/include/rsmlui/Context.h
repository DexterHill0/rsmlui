#pragma once
#include <RmlUi/Core/Context.h>

#include "rust/cxx.h"

namespace rsmlui {

inline auto context_update(Rml::Context* ctx) -> bool {
    return ctx->Update();
}

inline auto context_render(Rml::Context* ctx) -> bool {
    return ctx->Render();
}

inline auto context_load_document(Rml::Context* ctx, rust::String document_path)
    -> Rml::ElementDocument* {
    return ctx->LoadDocument(document_path.c_str());
}

inline void
context_set_dimensions(Rml::Context* ctx, Rml::Vector2i dimensions) {
    return ctx->SetDimensions(dimensions);
}
} // namespace rsmlui