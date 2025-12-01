use std::ops::{Deref, DerefMut};

use glam::IVec2;

use crate::core::context::Context;
use crate::errors::RsmlUiError;
use crate::interfaces::RawInterface;
use crate::interfaces::renderer::RenderInterfaceMarker;
use crate::interfaces::system::SystemInterfaceMarker;
use crate::platforms::DefaultPlatformInterface;
use crate::renderers::DefaultRenderInterface;
use crate::utils::input::KeyCode;

#[derive(Clone, Copy, Debug, Default)]
pub struct BackendOptions {
    pub allow_resize: bool,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ProcessEventsOptions {
    pub power_save: bool,
}

pub struct ProcessEventContext<'ctx> {
    pub context: &'ctx mut Context<'ctx>,
    pub key: KeyCode,
    pub key_modifier: i32,
    pub native_dp_ratio: f32,
    pub priority: bool,
}

// pub struct BackendGuard<B: Backend> {
//     pub(crate) backend: B,
// }

// impl<B: Backend> BackendGuard<B> {
//     pub(crate) fn new(backend: B) -> Self {
//         Self { backend }
//     }
// }

// impl<B: Backend> Deref for BackendGuard<B> {
//     type Target = B;

//     fn deref(&self) -> &Self::Target {
//         &self.backend
//     }
// }

// impl<B: Backend> DerefMut for BackendGuard<B> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.backend
//     }
// }

// impl<B: Backend> Drop for BackendGuard<B> {
//     fn drop(&mut self) {
//         self.backend.shutdown();
//     }
// }

// /// Wraps the user backend in a `BackendGuard` which implements drop to shutdown the backend when it's dropped
// pub(crate) trait BackendInternal: Backend {
//     fn to_guard(backend: Self) -> BackendGuard<Self>
//     where
//         Self: Sized,
//     {
//         BackendGuard::new(backend)
//     }

//     //     fn get_system_interface(&self) -> Option<Self::SystemInterface> {}
//     // fn get_render_interface(&self) -> Option<Self::RenderInterface> {}
// }

// impl<B: Backend> BackendInternal for B {}

// TODO: allow default interfaces somehow
pub trait Backend {
    type SystemInterface;
    type RenderInterface;

    fn initialize_with_options<T: Into<String>>(
        window_name: T,
        dimensions: IVec2,
        options: BackendOptions,
    ) -> Result<Self, RsmlUiError>
    where
        Self: Sized;

    fn initialize<T: Into<String>>(window_name: T, dimensions: IVec2) -> Result<Self, RsmlUiError>
    where
        Self: Sized,
    {
        Self::initialize_with_options(window_name, dimensions, BackendOptions::default())
    }

    fn get_system_interface(&mut self) -> Option<&mut Self::SystemInterface>;
    fn get_render_interface(&mut self) -> Option<&mut Self::RenderInterface>;

    fn request_exit(&self);

    fn begin_frame(&self);
    fn present_frame(&self);
}
