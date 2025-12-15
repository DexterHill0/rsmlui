#pragma once
#include <RmlUi/Core/Context.h>

#include "rust/cxx.h"

namespace rsmlui {
inline void context_destructor(Rml::Context* ctx) {
    ctx->~Context();
}

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
} // namespace rsmlui