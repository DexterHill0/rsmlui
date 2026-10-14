#pragma once

#include <RmlUi/Core.h>

#include <vector>

#include "RmlUi/Core/DecorationTypes.h"
#include "RmlUi/Core/Types.h"
#include "RmlUi/Core/Variant.h"
#include "rust/cxx.h"

namespace rsmlui {

// `Rml::Variant` is neither trivially movable nor trivially destructible, so it can't be
// handed to Rust by value.
// We allocate instead to get a stable pointer that can be passed to rust.
// I think we can get around that by passing around opaque blobs of sizeof(Variant)
// but unless allocation proves a performance issue this is easier for now.
inline auto new_variant() -> Rml::Variant* {
    return new Rml::Variant();
}

inline void variant_destructor(Rml::Variant* variant) {
    delete variant;
}

inline auto variant_get_type(const Rml::Variant* variant)
    -> Rml::Variant::Type {
    return variant->Rml::Variant::GetType();
}

inline void variant_clear(Rml::Variant* variant) {
    variant->Rml::Variant::Clear();
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

inline void variant_set_bool(Rml::Variant* variant, bool value) {
    // Use operator= instead of `Set` as `Set` does not clear the previous value.
    *variant = value;
}

inline void variant_set_byte(Rml::Variant* variant, uint8_t value) {
    *variant = value;
}

inline void variant_set_char(Rml::Variant* variant, int8_t value) {
    *variant = static_cast<char>(value);
}

inline void variant_set_float(Rml::Variant* variant, float value) {
    *variant = value;
}

inline void variant_set_double(Rml::Variant* variant, double value) {
    *variant = value;
}

inline void variant_set_int(Rml::Variant* variant, int32_t value) {
    *variant = value;
}

inline void variant_set_int64(Rml::Variant* variant, int64_t value) {
    *variant = value;
}

inline void variant_set_uint(Rml::Variant* variant, uint32_t value) {
    *variant = value;
}

inline void variant_set_uint64(Rml::Variant* variant, uint64_t value) {
    *variant = value;
}

inline void variant_set_vector2f(Rml::Variant* variant, Rml::Vector2f value) {
    *variant = value;
}

inline void variant_set_vector3f(Rml::Variant* variant, Rml::Vector3f value) {
    *variant = value;
}

inline void variant_set_vector4f(Rml::Variant* variant, Rml::Vector4f value) {
    *variant = value;
}

inline void variant_set_colorb(Rml::Variant* variant, Rml::Colourb value) {
    *variant = value;
}

inline void variant_set_color_stop_list(
    Rml::Variant* variant,
    rust::Slice<const Rml::ColorStop> value
) {
    *variant = Rml::ColorStopList(value.begin(), value.end());
}

inline void variant_set_str(Rml::Variant* variant, rust::Str value) {
    *variant = Rml::String(value);
}

inline void
variant_set_variant(Rml::Variant* variant, const Rml::Variant* value) {
    *variant = *value;
}

} // namespace rsmlui
