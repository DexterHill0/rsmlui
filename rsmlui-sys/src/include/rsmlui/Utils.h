#pragma once

#include <RmlUi/Core/Log.h>

#include <cstddef>
#include <type_traits>

namespace rsmlui {
using c_void = void;
}

namespace rsmlui {
#define ASSERT_INTERFACE_LAYOUT(TYPE) \
    static_assert( \
        sizeof(TYPE) == sizeof(InterfaceBridgeLayout), \
        #TYPE " size differs from InterfaceBridgeLayout" \
    ); \
    static_assert( \
        alignof(TYPE) == alignof(InterfaceBridgeLayout), \
        #TYPE " alignment differs from InterfaceBridgeLayout" \
    ); \
    static_assert( \
        std::is_polymorphic_v<TYPE>, \
        #TYPE " must remain polymorphic" \
    ); \
    static_assert( \
        std::is_same_v< \
            decltype(TYPE::rust_meta), \
            decltype(InterfaceBridgeLayout::rust_meta)>, \
        #TYPE "::rust_meta type differs from InterfaceBridgeLayout" \
    ); \
    static_assert( \
        std::is_same_v< \
            decltype(TYPE::rust_data), \
            decltype(InterfaceBridgeLayout::rust_data)>, \
        #TYPE "::rust_data type differs from InterfaceBridgeLayout" \
    )
} // namespace rsmlui

namespace rsmlui::log {
using RmlLogType = Rml::Log::Type;
}
