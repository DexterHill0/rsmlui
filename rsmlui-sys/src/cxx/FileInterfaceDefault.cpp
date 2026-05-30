#include "FileInterfaceDefault.h"

#include "rsmlui/FileInterface.h"

namespace rsmlui::file_interface {

auto new_default_file_interface() -> Rml::FileInterface* {
    return new Rml::FileInterfaceDefault();
}

void default_file_interface_destructor(Rml::FileInterface* interface) {
    delete interface;
}

} // namespace rsmlui::file_interface
