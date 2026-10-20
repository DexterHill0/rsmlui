// Only exists for bindgen to generate bindings

#pragma once
#include <RmlUi/Core.h>

#include <cstdint>

namespace Rml {
using Log_Type = Rml::Log::Type;
using XmlDataType = Rml::XMLDataType;
};

namespace Misc {
static int32_t STDIO_SEEK_CUR = SEEK_CUR;
static int32_t STDIO_SEEK_END = SEEK_END;
static int32_t STDIO_SEEK_SET = SEEK_SET;
} // namespace Misc