use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::time::{Duration, Instant};

use glam::IVec2;

use crate::core::app::{AppDriver, ApplicationHandler};
use crate::core::context::Context;
use crate::errors::RsmlUiError;
use crate::interfaces::renderer::{RenderInterface, RenderInterfaceMarker};
use crate::interfaces::system::{SystemInterface, SystemInterfaceMarker};
use crate::interfaces::window::WindowInterface;
use crate::interfaces::{InterfaceHandle, InterfaceInstancer, InterfaceState, RawInterface};

pub(crate) mod sealed {
    #[doc(hidden)]
    pub trait Sealed {}
}

pub(crate) trait BackendRuntime<T: 'static = ()>: sealed::Sealed {
    fn app_driver(&mut self) -> Box<dyn AppDriver<T>>;

    fn initialize(&mut self) -> Result<(), RsmlUiError>;
    /// # Safety
    ///
    /// The caller must ensure [`BackendRuntime::initialize`] has been called.
    unsafe fn begin_frame(&mut self);
    /// # Safety
    ///
    /// The caller must ensure [`BackendRuntime::initialize`] has been called.
    unsafe fn present_frame(&mut self);
}

pub trait MonolithicBackend<T: 'static>: BackendRuntime<T> {}

#[derive(Clone, Copy, Debug, Default)]
pub struct BackendOptions {
    pub allow_resize: bool,
    pub power_save: bool,
}

pub struct Backend<W, S, R, T: 'static = ()> {
    window: W,
    system: Option<InterfaceHandle<S>>,
    render: Option<InterfaceHandle<R>>,
    _phantom: PhantomData<T>,
}

impl<T: 'static, W: WindowInterface<T>> Backend<W, (), (), T> {
    pub fn new_with_window(window: W) -> Self {
        Self {
            window,
            system: None,
            render: None,
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static, W, R> Backend<W, (), R, T> {
    pub fn with_system_uninstanced<S2: 'static>(self, system_interface: S2) -> Backend<W, S2, R, T>
    where
        InterfaceState<S2>: SystemInterface,
    {
        Backend {
            window: self.window,
            system: Some(system_interface.instance()),
            render: self.render,
            _phantom: PhantomData,
        }
    }

    pub fn with_system<S2: 'static>(
        self,
        system_interface: InterfaceHandle<S2>,
    ) -> Backend<W, S2, R, T>
    where
        InterfaceHandle<S2>: SystemInterface,
    {
        Backend {
            window: self.window,
            system: Some(system_interface),
            render: self.render,
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static, W, S> Backend<W, S, (), T> {
    // pub fn with_render_uninstanced<R2: 'static>(self, render_interface: R2) -> Backend<W, S, R2, T>
    // where
    //     InterfaceState<R2>: RenderInterface,
    // {
    //     Backend {
    //         window: self.window,
    //         system: self.system,
    //         render: Some(render_interface.instance()),
    //         _phantom: PhantomData,
    //     }
    // }

    pub fn with_render<R2: 'static>(
        self,
        render_interface: InterfaceHandle<R2>,
    ) -> Backend<W, S, R2, T>
    where
        InterfaceHandle<R2>: RenderInterface,
    {
        Backend {
            window: self.window,
            system: self.system,
            render: Some(render_interface),
            _phantom: PhantomData,
        }
    }
}

impl<T: 'static, W, S, R> sealed::Sealed for Backend<W, S, R, T>
where
    W: WindowInterface<T>,
    InterfaceState<S>: SystemInterface,
    InterfaceState<R>: RenderInterface,
{
}

impl<T: 'static, W, S, R> BackendRuntime<T> for Backend<W, S, R, T>
where
    W: WindowInterface<T>,
    InterfaceState<S>: SystemInterface,
    InterfaceState<R>: RenderInterface,
{
    fn initialize(&mut self) -> Result<(), RsmlUiError> {
        // self.window.initialize()?;

        // if let Some(system) = &self.system {
        //     unsafe {
        //         rsmlui_sys::core::set_system_interface(system.class_ptr());
        //     }
        // }

        // if let Some(render) = &self.render {
        //     unsafe {
        //         rsmlui_sys::core::set_render_interface(render.class_ptr());
        //     }
        // }

        todo!();

        // Ok(())
    }

    fn app_driver(&mut self) -> Box<dyn AppDriver<T>> {
        self.window.driver()
    }

    unsafe fn begin_frame(&mut self) {
        todo!()
        // self.window.begin_frame();
    }

    unsafe fn present_frame(&mut self) {
        todo!()
        // self.window.present_frame();
    }
}
