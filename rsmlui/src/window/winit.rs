use std::marker::PhantomData;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent as WinitEvent;
pub use winit::event_loop::ControlFlow;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use crate::core::app::AppDriver;
use crate::core::core::AppDispatcher;
use crate::errors::RsmlUiError;
use crate::interfaces::window::WindowInterface;

pub type WinitWindowOptions = winit::window::WindowAttributes;

struct WinitWindowInner<T: 'static = ()> {
    window_options: WinitWindowOptions,
    window: Option<Window>,

    // sender: Sender<WindowEvent<T>>,
    last_frame: Instant,
    _phantom: PhantomData<T>,
}

impl<T: 'static> ApplicationHandler<T> for WinitWindowInner<T> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(self.window_options.clone())
                .expect("failed to create window"),
        );
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WinitEvent,
    ) {
        match event {
            WinitEvent::CloseRequested => {
                // let _ = self.sender.send(WindowEvent::ExitRequested);
            },

            _ => {},
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        self.last_frame = now;

        // let _ = self.sender.send(WindowEvent::RenderRequested(delta));
    }
}

pub struct WinitWindow<T: 'static = ()> {
    event_loop: EventLoop<T>,
    window_options: WinitWindowOptions,
    // sender: Option<Sender<WindowEvent<T>>>,
}

impl WinitWindow<()> {
    pub fn new_with_options(
        control_flow: ControlFlow,
        options: WinitWindowOptions,
    ) -> Result<Self, RsmlUiError> {
        WinitWindow::new_custom_with_options(control_flow, options)
    }
}

impl<T: 'static> WinitWindow<T> {
    pub fn new_custom_with_options(
        control_flow: ControlFlow,
        options: WinitWindowOptions,
    ) -> Result<WinitWindow<T>, RsmlUiError> {
        let event_loop = EventLoop::<T>::with_user_event()
            .build()
            .map_err(RsmlUiError::from)?;

        event_loop.set_control_flow(control_flow);

        Ok(WinitWindow::<T> {
            event_loop,
            window_options: options,
            // sender: None,
        })
    }
}

impl<T: 'static> AppDriver<T> for WinitWindow<T> {
    fn run(self: Box<Self>, dispatcher: &mut AppDispatcher<T>) -> Result<(), RsmlUiError> {
        // let (tx, _rx) = std::sync::mpsc::channel();

        // self.sender = Some(sender.clone());

        let mut inner = WinitWindowInner::<T> {
            window_options: self.window_options.clone(),
            window: None,
            // sender,
            last_frame: Instant::now(),
            _phantom: PhantomData,
        };

        dispatcher.starting()?;

        self.event_loop.run_app(&mut inner)?;

        Ok(())
    }
}

impl<T: 'static> WindowInterface<T> for WinitWindow<T> {
    fn initialize(&mut self) -> Result<(), RsmlUiError> {
        Ok(())
    }

    fn driver(&mut self) -> Box<dyn AppDriver<T>> {
        // self
        todo!()
    }
}
