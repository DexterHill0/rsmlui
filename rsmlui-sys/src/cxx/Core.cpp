#include <RmlUi/Core/Core.h>

#include "rsmlui/Core.h"

namespace rsmlui {
rust::String get_version() {
  Rml::String ver = Rml::GetVersion();
  return rust::String(ver.c_str());
}
} // namespace rsmlui