#pragma once

#include <RmlUi/Core.h>

#include "RmlUi/Core/Types.h"
#include "RmlUi/Core/Variant.h"
#include "rust/cxx.h"

namespace rsmlui {

inline auto variant_get_type(const Rml::Variant* variant)
    -> Rml::Variant::Type {
    return variant->Rml::Variant::GetType();
}

inline auto variant_as_bool(const Rml::Variant* variant) -> const bool& {
    return variant->GetReference<bool>();
}

inline auto variant_as_byte(const Rml::Variant* variant) -> const uint8_t& {
    return reinterpret_cast<const uint8_t&>(variant->GetReference<Rml::byte>());
}

inline auto variant_as_char(const Rml::Variant* variant) -> const int8_t& {
    return reinterpret_cast<const int8_t&>(variant->GetReference<char>());
}

inline auto variant_as_float(const Rml::Variant* variant) -> const float& {
    return variant->GetReference<float>();
}

inline auto variant_as_double(const Rml::Variant* variant) -> const double& {
    return variant->GetReference<double>();
}

inline auto variant_as_int(const Rml::Variant* variant) -> const int32_t& {
    return reinterpret_cast<const int32_t&>(variant->GetReference<int>());
}

inline auto variant_as_int64(const Rml::Variant* variant) -> const int64_t& {
    return variant->GetReference<int64_t>();
}

inline auto variant_as_uint(const Rml::Variant* variant) -> const uint32_t& {
    return reinterpret_cast<const uint32_t&>(
        variant->GetReference<unsigned int>()
    );
}

inline auto variant_as_uint64(const Rml::Variant* variant) -> const uint64_t& {
    return variant->GetReference<uint64_t>();
}

inline auto variant_as_vector2f(const Rml::Variant* variant)
    -> const Rml::Vector2f& {
    return variant->GetReference<Rml::Vector2f>();
}

inline auto variant_as_vector3f(const Rml::Variant* variant)
    -> const Rml::Vector3f& {
    return variant->GetReference<Rml::Vector3f>();
}

inline auto variant_as_vector4f(const Rml::Variant* variant)
    -> const Rml::Vector4f& {
    return variant->GetReference<Rml::Vector4f>();
}

inline auto variant_as_colorb(const Rml::Variant* variant)
    -> const Rml::Colourb& {
    return variant->GetReference<Rml::Colourb>();
}

inline auto variant_as_color_stop_list(const Rml::Variant* variant)
    -> rust::Slice<const Rml::ColorStop> {
    const auto& vec = variant->GetReference<Rml::Vector<Rml::ColorStop>>();

    return {vec.data(), vec.size()};
}

inline auto variant_as_str(const Rml::Variant* variant) -> rust::Str {
    const auto& str = variant->GetReference<std::string>();

    return {str};
}

} // namespace rsmlui
