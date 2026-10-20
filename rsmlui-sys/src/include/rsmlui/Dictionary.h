#pragma once

#include <RmlUi/Core.h>
#include <__iterator/wrap_iter.h>

#include "RmlUi/Core/Types.h"
#include "RmlUi/Core/Variant.h"
#include "rsmlui/Utils.h"
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

inline auto dictionary_iterator_begin(const Rml::Dictionary* dictionary)
    -> const rsmlui::c_void* {
    return reinterpret_cast<const rsmlui::c_void*>(dictionary->begin().base());
}

inline auto dictionary_iterator_next(
    const Rml::Dictionary* dictionary,
    const rsmlui::c_void* iter_base
) -> DictionaryIterPair {
    const auto* dict_iter_base =
        reinterpret_cast<const Rml::Dictionary::const_iterator::iterator_type>(
            iter_base
        );

    auto distance = std::distance(dictionary->begin().base(), dict_iter_base);
    auto iter = dictionary->begin() + distance;

    if (iter == dictionary->end()) {
        return DictionaryIterPair {
            .finished = true,
            .key = rust::Str(""),
            .value = nullptr,
            .iter_base = iter_base
        };
    }

    rust::Str key = {iter->first};
    const Rml::Variant* value = &iter->second;

    // Move the iterator forward so `iter.base()` returns the next position we want to
    // iterate from.
    iter++;

    auto pair = DictionaryIterPair {
        .finished = false,
        .key = key,
        .value = value,
        .iter_base = iter.base(),
    };

    iter++;

    return pair;
}

inline auto dictionary_iterator_count(const Rml::Dictionary* dictionary)
    -> size_t {
    return dictionary->size();
}

inline auto dictionary_iterator_size_hint(
    const Rml::Dictionary* dictionary,
    const rsmlui::c_void* iter_base
) -> DictionaryIterSizeHint {
    const auto* dict_iter_base =
        reinterpret_cast<const Rml::Dictionary::const_iterator::iterator_type>(
            iter_base
        );

    auto consumed = std::distance(dictionary->begin().base(), dict_iter_base);

    return DictionaryIterSizeHint {
        .lower = dictionary->size() - consumed,
        .upper = dictionary->size() - consumed,
    };
}

} // namespace rsmlui