#pragma once

#include <RmlUi/Core.h>

#include "RmlUi/Core/Types.h"
#include "RmlUi/Core/Variant.h"
#include "rust/cxx.h"

namespace rsmlui {
inline auto
dictionary_get_variant(const Rml::Dictionary* dictionary, rust::Str key)
    -> const Rml::Variant* {
    return Rml::GetIf(*dictionary, std::string(key.data(), key.size()));
}

} // namespace rsmlui