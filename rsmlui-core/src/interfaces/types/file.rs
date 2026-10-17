use std::mem::transmute;
use std::path::Path;

use rsmlui_macros::rmldoc;
use rsmlui_sys::core;
use rsmlui_sys::file_interface::{
    FileInterfaceBridge, RmlFileInterface, RustFileInterface, file_interface_default_length,
    file_interface_default_load_file, new_rust_file_interface, rust_file_interface_destructor,
};
use rsmlui_sys::interfaces::Opaque;
use sealed::sealed;
use thiserror::Error;

use crate::_private::HasOwnedInterface;
use crate::interfaces::bridge::BridgedHandle;
use crate::interfaces::object::InterfaceObject;
use crate::types::handles::FileHandle;
use crate::utils::conversions::{FromSys, IntoSys};
use crate::utils::private::Uncallable;

pub type FileInterfaceObject<T> = InterfaceObject<T, RustFileInterface>;

/// A custom [`Rml::FileInterface`].
///
/// [`Rml::FileInterface`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/file.html
pub type FileInterface<T> = BridgedHandle<T, RustFileInterface>;

#[sealed]
unsafe impl<T: FileInterfaceBehaviour> crate::interfaces::bridge::BridgeOps<RustFileInterface>
    for T
{
    fn construct(handle: &mut FileInterfaceObject<Self>) {
        // The fat pointer data component is the address of the heap-allocated `InterfaceObject`.
        // That address is stable for the lifetime of the `BridgedHandle`.
        let fat_ptr: *mut dyn FileInterfaceBridge = handle;

        let (data, meta) = fat_ptr.to_raw_parts();

        let meta_raw: *const () = unsafe { transmute(meta) };

        let cpp =
            unsafe { new_rust_file_interface(meta_raw as *const Opaque, data as *mut Opaque) };

        handle.bridge = cpp;
    }

    unsafe fn destroy_bridge(bridge: *mut RustFileInterface) {
        // Safety: `bridge` is a valid, non-null pointer
        unsafe { rust_file_interface_destructor(bridge) }
    }

    unsafe fn assert_not_registered(bridge: *mut RustFileInterface) {
        let current_interface_ptr = core::get_file_interface();

        // `RustFileInterface` inherits from `FileInterface` in C++
        // so the cast is safe.
        let self_ptr = bridge.cast();

        assert_ne!(
            current_interface_ptr, self_ptr,
            "file interface dropped while still registered"
        );
    }
}

/// Errors produced by the default [`FileInterface`] method implementations.
#[derive(Error, Debug)]
pub enum FileError {
    #[error("failed to open file")]
    OpenFailed,
    #[error("failed to read file")]
    ReadFailed,
    #[error("failed to seek in file")]
    SeekFailed,
    #[error("failed to load file")]
    LoadFailed,
    #[error("invalid path")]
    InvalidPath,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SeekOffset {
    Beginning(usize),
    Current(usize),
    End(isize),
}

/// Implement this trait to add behaviour ot a custom [`Rml::FileInterface`].
///
/// Some methods require implementations, while some have default implementations.
///
/// Methods receive `self: &mut FileInterfaceObject<Self>` rather than `&mut self` in order
/// to ensure correct initialisation and lifetime of values.
///
/// ## Dyn-compatibility
///
/// Every method is marked `where Self: Sized`, which keeps the trait dyn-compatible. Therefore,
/// methods are not callable through a `dyn FileInterface` trait object.
///
/// [`Rml::FileInterface`]: https://mikke89.github.io/RmlUiDoc/pages/cpp_manual/interfaces/file.html
#[rmldoc(file = "api_Rml-FileInterface.md", name = "Rml::FileInterface")]
pub trait FileInterfaceBehaviour {
    type Error: std::error::Error + From<FileError>;

    #[rmldoc(name = "Rml::FileInterface::Open")]
    fn open(self: &mut FileInterfaceObject<Self>, path: &Path) -> Result<FileHandle, Self::Error>
    where
        Self: Sized;

    #[rmldoc(name = "Rml::FileInterface::Close")]
    fn close(self: &mut FileInterfaceObject<Self>, file: FileHandle) -> Result<(), Self::Error>
    where
        Self: Sized;

    #[rmldoc(name = "Rml::FileInterface::Read")]
    fn read(
        self: &mut FileInterfaceObject<Self>,
        file: FileHandle,
        buf: &mut [u8],
    ) -> Result<usize, Self::Error>
    where
        Self: Sized;

    #[rmldoc(name = "Rml::FileInterface::Seek")]
    fn seek(
        self: &mut FileInterfaceObject<Self>,
        file: FileHandle,
        offset: SeekOffset,
    ) -> Result<(), Self::Error>
    where
        Self: Sized;

    #[rmldoc(name = "Rml::FileInterface::Tell")]
    fn tell(self: &mut FileInterfaceObject<Self>, file: FileHandle) -> usize
    where
        Self: Sized;

    #[rmldoc(name = "Rml::FileInterface::Length")]
    fn length(self: &mut FileInterfaceObject<Self>, file: FileHandle) -> usize
    where
        Self: Sized,
    {
        unsafe { file_interface_default_length(self.bridge_ptr(), file.into_sys()) }
    }

    #[rmldoc(name = "Rml::FileInterface::LoadFile")]
    fn load_file(self: &mut FileInterfaceObject<Self>, path: &Path) -> Result<Vec<u8>, Self::Error>
    where
        Self: Sized,
    {
        let mut out = Vec::new();

        let ok = unsafe {
            file_interface_default_load_file(
                self.bridge_ptr(),
                path.to_str().ok_or(FileError::InvalidPath)?,
                &mut out,
            )
        };

        if ok {
            Ok(out)
        } else {
            Err(FileError::LoadFailed.into())
        }
    }
}

// The sys crate uses `FileInterfaceBridge` as the dispatch from C++ to Rust.
// This impl forwards each call to the user's `FileInterface` implementation.
unsafe impl<T: FileInterfaceBehaviour> FileInterfaceBridge for FileInterfaceObject<T> {
    unsafe fn open(&mut self, path: &str) -> rsmlui_sys::Rml_FileHandle {
        match T::open(self, Path::new(path)) {
            Ok(handle) => handle.into_sys(),
            Err(error) => {
                crate::error!("[FileInterface] failed to open file {}: {:?}", path, error);

                FileHandle::INVALID.into_sys()
            },
        }
    }

    #[inline]
    unsafe fn close(&mut self, file: rsmlui_sys::Rml_FileHandle) {
        match T::close(self, FromSys::from_sys(file)) {
            Ok(..) => {},
            Err(error) => {
                crate::error!("[FileInterface] failed to close file: {:?}", error);
            },
        }
    }

    #[inline]
    unsafe fn read(
        &mut self,
        buffer: *mut u8,
        size: usize,
        file: rsmlui_sys::Rml_FileHandle,
    ) -> usize {
        // Safety: the caller allocates a buffer of exactly `size` bytes and  passes
        // that same value as `size`.
        let buf = unsafe { std::slice::from_raw_parts_mut(buffer, size) };

        #[allow(clippy::manual_unwrap_or_default, clippy::manual_unwrap_or)]
        match T::read(self, FromSys::from_sys(file), buf) {
            Ok(n) => n,
            Err(error) => {
                crate::error!("[FileInterface] failed to read file: {:?}", error);

                0
            },
        }
    }

    #[inline]
    unsafe fn seek(&mut self, file: rsmlui_sys::Rml_FileHandle, offset: i64, origin: i32) -> bool {
        let seek = match origin {
            rsmlui_sys::Misc_STDIO_SEEK_SET => SeekOffset::Beginning(offset as usize),
            rsmlui_sys::Misc_STDIO_SEEK_CUR => SeekOffset::Current(offset as usize),
            rsmlui_sys::Misc_STDIO_SEEK_END => SeekOffset::End(offset as isize),
            _ => panic!("unknown seek origin: {origin}"),
        };

        match T::seek(self, FromSys::from_sys(file), seek) {
            Ok(..) => true,
            Err(error) => {
                crate::error!("[FileInterface] failed to seek file: {:?}", error);

                false
            },
        }
    }

    #[inline]
    unsafe fn tell(&mut self, file: rsmlui_sys::Rml_FileHandle) -> usize {
        T::tell(self, FromSys::from_sys(file))
    }

    #[inline]
    unsafe fn length(&mut self, file: rsmlui_sys::Rml_FileHandle) -> usize {
        T::length(self, FromSys::from_sys(file))
    }

    #[inline]
    unsafe fn load_file(&mut self, path: &str, out_data: &mut Vec<u8>) -> bool {
        match T::load_file(self, Path::new(path)) {
            Ok(contents) => {
                out_data.extend_from_slice(contents.as_ref());

                true
            },
            Err(error) => {
                crate::error!("[FileInterface] failed to load file {}: {:?}", path, error);

                false
            },
        }
    }
}

// Using `RmlFileInterface` so it's consistent with existing C++ file interfaces.
// `RustFileInterface` is simply a subclass of `RmlFileInterface` so it represents
// the same kind of object.
impl<T: FileInterfaceBehaviour> crate::interfaces::object::IntoRawInterface<RmlFileInterface>
    for &mut FileInterface<T>
{
    fn into_raw(self, _: Uncallable) -> *mut RmlFileInterface {
        self.bridge.cast()
    }
}

impl<T: FileInterfaceBehaviour> HasOwnedInterface<RmlFileInterface> for T {
    type Owned = FileInterface<T>;
}
