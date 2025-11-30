use std::ops::{Deref, DerefMut};

use glam::IVec2;

use crate::{
    core::context::Context,
    errors::RsmlUiError,
    interfaces::{
        renderer::{IntoRenderInterfacePtr, RenderInterface},
        system::{IntoSystemInterfacePtr, SystemInterface},
    },
    utils::input::KeyCode,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BackendOptions {
    pub allow_resize: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProcessEventsOptions {
    pub power_save: bool,
}

pub struct ProcessEventContext<'a> {
    pub context: &'a mut Context,
    pub key: KeyCode,
    pub key_modifier: i32,
    pub native_dp_ratio: f32,
    pub priority: bool,
}

pub struct BackendGuard<B: Backend>(pub(crate) B);

impl<B: Backend> BackendGuard<B> {
    pub fn new(backend: B) -> Self {
        Self(backend)
    }
}

impl<B: Backend> Deref for BackendGuard<B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: Backend> DerefMut for BackendGuard<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<B: Backend> Drop for BackendGuard<B> {
    fn drop(&mut self) {
        self.0.shutdown();
    }
}

pub trait Backend {
    type SystemInterface: IntoSystemInterfacePtr;
    type RenderInterface: IntoRenderInterfacePtr;

    fn initialize_with_options<T: Into<String>>(
        window_name: T,
        dimensions: IVec2,
        options: BackendOptions,
    ) -> Result<BackendGuard<Self>, RsmlUiError>
    where
        Self: Sized;

    fn initialize<T: Into<String>>(
        window_name: T,
        dimensions: IVec2,
    ) -> Result<BackendGuard<Self>, RsmlUiError>
    where
        Self: Sized,
    {
        Self::initialize_with_options(window_name, dimensions, BackendOptions::default())
    }

    fn shutdown(&mut self);

    fn get_system_interface(&mut self) -> Option<Self::SystemInterface>;
    fn get_render_interface(&mut self) -> Option<Self::RenderInterface>;

    fn set_event_callback(
        &mut self,
        callback: impl FnMut(ProcessEventContext<'_>) -> bool + 'static,
    );

    fn process_events(&mut self, context: &mut Context, options: ProcessEventsOptions) -> bool;

    fn request_exit(&mut self);

    fn begin_frame(&mut self);
    fn present_frame(&mut self);
}
