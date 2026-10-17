#pragma once

#include <RmlUi/Core/Log.h>

namespace rsmlui::log {
using RmlLogType = Rml::Log::Type;
}

#define ASSERT_INTERFACE_LAYOUT(TYPE) \
    static_assert( \
        sizeof(TYPE) == sizeof(void*) * 3 && alignof(TYPE) == alignof(void*), \
        #TYPE " no longer matches InterfaceBridgeLayout" \
    ); \
    static_assert( \
        offsetof(TYPE, rust_meta) == sizeof(void*), \
        #TYPE "::rust_meta is no longer the second pointer" \
    ); \
    static_assert( \
        offsetof(TYPE, rust_data) == sizeof(void*) * 2, \
        #TYPE "::rust_data is no longer the third pointer" \
    );