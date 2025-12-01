use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::path::PathBuf;

use glam::Vec2;
use rsmlui_sys::interfaces::{InterfaceOpaquePtr, ThinInterface};
use rsmlui_sys::system_interface::{
    RawSystemInterface, RmlSystemInterface, new_rust_system_interface,
    rust_system_interface_destructor,
};

use crate::core::log::LogLevel;
use crate::interfaces::sealed::Sealed;
use crate::interfaces::{
    self, InterfaceHandle, InterfaceInstancer, InterfaceMarker, InterfaceState, RawInterface,
};
use crate::utils::conversions::{FromSys, IntoSys};
use crate::utils::cursor::Cursor;

pub struct SystemInterfaceMarker;

impl InterfaceMarker for SystemInterfaceMarker {
    type Ptr = *mut RmlSystemInterface;
}

impl<I> Into<RawInterface<SystemInterfaceMarker>> for &mut InterfaceHandle<I> {
    fn into(self) -> RawInterface<SystemInterfaceMarker> {
        RawInterface(self.class_ptr() as _, PhantomData)
    }
}

pub trait SystemInterfaceBehaviour: interfaces::sealed::Sealed {
    fn get_elapsed_time(&mut self) -> f64 {
        unsafe {
            rsmlui_sys::system_interface::system_interface_get_elapsed_time(self.class_ptr() as _)
        }
    }

    fn translate_string(&mut self, input: &str) -> String {
        unsafe {
            rsmlui_sys::system_interface::system_interface_translate_string(
                self.class_ptr() as _,
                input,
            )
        }
    }

    fn join_path(&mut self, document_path: PathBuf, path: PathBuf) -> PathBuf {
        unsafe {
            rsmlui_sys::system_interface::system_interface_join_path(
                self.class_ptr() as _,
                document_path.to_str().unwrap(),
                path.to_str().unwrap(),
            )
            .into()
        }
    }

    fn log_message(&mut self, level: LogLevel, message: &str) -> bool {
        unsafe {
            rsmlui_sys::system_interface::system_interface_log_message(
                self.class_ptr() as _,
                level,
                message,
            )
        }
    }

    fn set_mouse_cursor(&mut self, cursor_name: Cursor) {
        unsafe {
            rsmlui_sys::system_interface::system_interface_set_mouse_cursor(
                self.class_ptr() as _,
                cursor_name.into(),
            );
        }
    }

    fn set_clipboard_text(&mut self, text: &str) {
        unsafe {
            rsmlui_sys::system_interface::system_interface_set_clipboard_text(
                self.class_ptr() as _,
                text,
            );
        }
    }

    fn get_clipboard_text(&mut self) -> String {
        unsafe {
            rsmlui_sys::system_interface::system_interface_get_clipboard_text(self.class_ptr() as _)
        }
    }

    fn activate_keyboard(&mut self, caret_position: Vec2, line_height: f32) {
        unsafe {
            rsmlui_sys::system_interface::system_interface_activate_keyboard(
                self.class_ptr() as _,
                caret_position.into_sys(),
                line_height,
            );
        }
    }

    fn deactivate_keyboard(&mut self) {
        unsafe {
            rsmlui_sys::system_interface::system_interface_deactivate_keyboard(
                self.class_ptr() as _
            );
        }
    }
}

type ThinSystermInterface = ThinInterface<*mut dyn RawSystemInterface>;

impl<I: 'static> InterfaceInstancer for I
where
    InterfaceState<I>: SystemInterfaceBehaviour,
{
    type Output = InterfaceHandle<Self>;

    fn instance(self) -> Self::Output {
        let mut pinned = Box::pin(UnsafeCell::new(InterfaceState {
            value: self,
            class_ptr: std::ptr::null_mut(),
        }));

        let trait_obj_ref: &mut dyn RawSystemInterface = unsafe { &mut *pinned.as_mut().get() };

        let protected_ptr: *mut dyn RawSystemInterface = trait_obj_ref;

        let wrapped: ThinSystermInterface = Box::new(UnsafeCell::new(protected_ptr));

        let thin_ptr: Box<ThinSystermInterface> = Box::new(wrapped);

        debug_assert_eq!(
            std::mem::size_of::<InterfaceOpaquePtr>(),
            std::mem::size_of_val(&thin_ptr)
        );

        let raw_opaque = Box::into_raw(thin_ptr) as InterfaceOpaquePtr;

        let raw_cpp = unsafe { new_rust_system_interface(raw_opaque) };

        unsafe {
            (&mut *pinned.as_mut().get()).class_ptr = raw_cpp as InterfaceOpaquePtr;
        }

        InterfaceHandle {
            value: pinned,
            raw: raw_opaque,
            drop: |slf| unsafe {
                debug_assert!(!slf.raw.is_null());

                // delete the class in c++
                rust_system_interface_destructor(slf.class_ptr() as *mut RmlSystemInterface);

                // clean up memory in rust
                let boxed_ptr = slf.raw as *mut ThinSystermInterface;

                drop(Box::from_raw(boxed_ptr));
            },
        }
    }
}

impl<I> RawSystemInterface for InterfaceState<I>
where
    InterfaceState<I>: SystemInterfaceBehaviour,
{
    unsafe fn get_elapsed_time(&mut self) -> f64 {
        SystemInterfaceBehaviour::get_elapsed_time(self)
    }

    unsafe fn translate_string(&mut self, input: &str) -> String {
        SystemInterfaceBehaviour::translate_string(self, input)
    }

    unsafe fn join_path(&mut self, document_path: &str, path: &str) -> String {
        dbg!(&document_path, &path);

        SystemInterfaceBehaviour::join_path(self, document_path.into(), path.into())
            .to_str()
            .unwrap()
            .to_string()
    }

    unsafe fn log_message(&mut self, level: rsmlui_sys::Rml_Log_Type, message: &str) -> bool {
        SystemInterfaceBehaviour::log_message(self, level, message)
    }

    unsafe fn set_mouse_cursor(&mut self, cursor_name: &str) {
        SystemInterfaceBehaviour::set_mouse_cursor(self, cursor_name.into());
    }

    unsafe fn set_clipboard_text(&mut self, text: &str) {
        SystemInterfaceBehaviour::set_clipboard_text(self, text);
    }

    unsafe fn get_clipboard_text(&mut self) -> String {
        SystemInterfaceBehaviour::get_clipboard_text(self)
    }

    unsafe fn activate_keyboard(
        &mut self,
        caret_position: rsmlui_sys::Rml_Vector2f,
        line_height: f32,
    ) {
        SystemInterfaceBehaviour::activate_keyboard(
            self,
            Vec2::from_sys(caret_position),
            line_height,
        );
    }

    unsafe fn deactivate_keyboard(&mut self) {
        SystemInterfaceBehaviour::deactivate_keyboard(self);
    }
}
