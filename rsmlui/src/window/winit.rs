use std::time::Duration;

use glam::IVec2;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::run_on_demand::EventLoopExtRunOnDemand;
use winit::window::{Window, WindowAttributes, WindowId};

use crate::errors::RsmlUiError;
use crate::interfaces::window::WindowInterface;
use crate::types::cursor::Cursor;

pub type WinitWindowOptions = WindowAttributes;

struct WinitWindowInner {
    window_options: WinitWindowOptions,
    window: Option<Window>,
}

impl<T: 'static> ApplicationHandler<T> for WinitWindowInner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // self.window = Some(event_loop.create_window(self.window_options).unwrap());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        todo!()
    }
}

pub struct WinitWindow<T: 'static> {
    window_options: WinitWindowOptions,
    event_loop: EventLoop<T>,
    should_close: bool,

    window_inner: WinitWindowInner,
}

impl<T: 'static> WinitWindow<T> {
    pub fn new_with_options(
        control_flow: ControlFlow,
        options: WinitWindowOptions,
    ) -> Result<Self, RsmlUiError> {
        // let event_loop = EventLoop::<T>::with_user_event()
        //     .build()
        //     .map_err(RsmlUiError::from)?;

        // event_loop.set_control_flow(control_flow);

        // Ok(Self {
        //     window_options: options,
        //     event_loop,
        //     should_close: false,
        //     window_inner: WinitWindowInner {
        //         window_options: self.window_options.clone(),
        //         window: None,
        //     },
        // })
        todo!()
    }
}

impl<T: 'static> WindowInterface<T> for WinitWindow<T> {
    fn initialize(&mut self) -> Result<(), RsmlUiError> {
        // self.window = Some(WinitWindowInner {
        //     window_options: self.window_options.clone(),
        //     window: None,
        // });

        // self.event_loop.run_app_on_demand(app);
        // Ok(())
        todo!()
    }

    fn driver(&mut self) -> &mut dyn crate::core::app::AppDriver<T> {
        todo!()
    }

    // fn poll_events(
    //     &mut self,
    //     sender: &WindowEventEmitter<T>,
    //     delta: Duration,
    // ) -> Result<(), RsmlUiError> {
    //     todo!()
    // }

    // fn should_close(&self) -> bool {
    //     todo!()
    // }

    // fn begin_frame(&mut self) {
    //     todo!()
    // }

    // fn present_frame(&mut self) {
    //     todo!()
    // }

    // fn dimensions(&self) -> glam::IVec2 {
    //     todo!()
    // }

    // fn set_cursor(&mut self, cursor: Cursor) {
    //     todo!()
    // }
}
