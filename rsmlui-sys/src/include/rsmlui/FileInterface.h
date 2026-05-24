#pragma once
#include <RmlUi/Core/Core.h>
#include <RmlUi/Core/FileInterface.h>

#include <cstdint>

#include "./InterfaceDecls.h"
#include "RmlUi/Core/Types.h"
#include "rust/cxx.h"
#include "src/ffi/file_interface.rs.h"

namespace rsmlui::file_interface {

inline auto RustFileInterface::Open(const Rml::String& path)
    -> Rml::FileHandle {
    return rust_open(this, {path});
}

inline void RustFileInterface::Close(Rml::FileHandle file) {
    rust_close(this, file);
}

inline auto
RustFileInterface::Read(void* buffer, size_t size, Rml::FileHandle file)
    -> size_t {
    return rust_read(this, static_cast<uint8_t*>(buffer), size, file);
}

inline auto
RustFileInterface::Seek(Rml::FileHandle file, long offset, int origin) -> bool {
    return rust_seek(this, file, static_cast<int64_t>(offset), origin);
}

inline auto RustFileInterface::Tell(Rml::FileHandle file) -> size_t {
    return rust_tell(this, file);
}

inline auto RustFileInterface::Length(Rml::FileHandle file) -> size_t {
    return rust_length(this, file);
}

inline auto
RustFileInterface::LoadFile(const Rml::String& path, Rml::String& out_data)
    -> bool {
    rust::Vec<uint8_t> buf;

    bool result = rust_load_file(this, {path}, buf);

    out_data.assign(reinterpret_cast<const char*>(buf.data()), buf.size());

    return result;
}

inline auto new_rust_file_interface(
    const rsmlui::Opaque* rust_meta,
    rsmlui::Opaque* rust_data
) -> RustFileInterface* {
    return new RustFileInterface((void*)rust_meta, (void*)rust_data);
}

inline void rust_file_interface_destructor(RustFileInterface* obj) {
    delete obj;
}

// The following functions call the base `Rml::FileInterface` implementation directly.
// Only `Length` and `LoadFile` have non-pure-virtual base implementations; the other
// five methods are pure virtual and cannot be called on the base class.
inline auto file_interface_default_length(
    RustFileInterface* interface,
    Rml::FileHandle file
) -> size_t {
    return interface->Rml::FileInterface::Length(file);
}

inline auto file_interface_default_load_file(
    RustFileInterface* interface,
    rust::Str path,
    rust::Vec<uint8_t>& out_data
) -> bool {
    Rml::String buf;

    bool result =
        interface->Rml::FileInterface::LoadFile(Rml::String(path), buf);

    out_data = rust::Vec<uint8_t> {};

    for (char byte : buf) {
        out_data.push_back(static_cast<uint8_t>(byte));
    }

    return result;
}

} // namespace rsmlui::file_interface
