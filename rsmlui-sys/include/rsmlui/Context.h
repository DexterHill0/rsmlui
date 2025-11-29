#pragma once
#include <RmlUi/Core/Context.h>

#include "rsmlui/Types.h"
#include "rust/cxx.h"

namespace rsmlui {
inline auto context_update(Context* ctx) -> bool {
    return ctx->Update();
}

inline auto context_render(Context* ctx) -> bool {
    return ctx->Render();
}

inline auto context_load_document(Context* ctx, rust::String document_path)
    -> ElementDocument* {
    return ctx->LoadDocument(document_path.c_str());
}
} // namespace rsmlui