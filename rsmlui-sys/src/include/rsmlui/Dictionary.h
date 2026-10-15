#pragma once

#include <RmlUi/Core.h>

#include "RmlUi/Core/Types.h"
#include "RmlUi/Core/Variant.h"
#include "rust/cxx.h"
#include "src/ffi/dictionary.rs.h"

namespace rsmlui {
inline auto new_dictionary() -> Rml::Dictionary* {
    return new Rml::Dictionary();
}

inline auto dictionary_destructor(Rml::Dictionary* dictionary) {
    delete dictionary;
}

inline auto dictionary_insert(
    Rml::Dictionary* dictionary,
    rust::Str key,
    const Rml::Variant* value
) -> DictionaryInsert {
    auto inserted =
        dictionary->emplace(std::string(key.data(), key.size()), *value);

    if (inserted.second) {
        return DictionaryInsert {
            .failed = nullptr,
            .inserted = &inserted.first->second,
        };
    }

    return DictionaryInsert {
        .failed = &inserted.first->second,
        .inserted = nullptr,
    };
}

inline auto
dictionary_get_variant_const(const Rml::Dictionary* dictionary, rust::Str key)
    -> const Rml::Variant* {
    return Rml::GetIf(*dictionary, std::string(key.data(), key.size()));
}

inline auto dictionary_get_variant(Rml::Dictionary* dictionary, rust::Str key)
    -> Rml::Variant* {
    return Rml::GetIf(*dictionary, std::string(key.data(), key.size()));
}

} // namespace rsmlui