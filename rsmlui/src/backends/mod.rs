#[cfg(feature = "backend-win32-gl2")]
pub mod s_win32_r_gl2;

#[cfg(any(feature = "backend-win32-gl2"))]
mod driver {
    use std::cell::RefCell;
    use std::sync::mpsc::channel;

    use crate::core::app::AppDriver;
    use crate::core::core::AppDispatcher;
    use crate::core::events::{KeyboardEvent, WindowEvent};
    use crate::errors::RsmlUiError;
    use crate::types::input::{KeyCode, KeyModifier};
    use crate::utils::raw::Raw;

    // impractical, but while RmlUI doesn't have a user data pointer in the callback, this is required
    // FIXME: remove once RmlUi has user data pointer
    thread_local! {
        pub(crate) static BACKEND_EVENTS_CALLBACK: RefCell<
            Option<Box<dyn for<'ctx> FnMut(KeyCode, KeyModifier, f32, bool) -> bool>>
        > = RefCell::new(None);
    }

    pub struct MonolithicBackendDriver {
        pub(crate) power_save: bool,
    }

    impl<T: 'static> AppDriver<T> for MonolithicBackendDriver {
        fn run(&mut self, app: &mut AppDispatcher<T>) -> Result<(), RsmlUiError> {
            app.starting()?;

            let (tx, rx) = channel::<WindowEvent<T>>();

            let tx_inner = tx.clone();

            BACKEND_EVENTS_CALLBACK.replace(Some(Box::new(
                move |code, modifier, native_dp_ratio, priority| {
                    // FIXME: remove expect?
                    tx_inner
                        .clone()
                        .send(WindowEvent::KeyboardEvent(KeyboardEvent::KeyPressed {
                            code,
                            modifier,
                            native_dp_ratio,
                            fallback: !priority,
                        }))
                        .expect("failed to send KeyPress event");

                    true
                },
            )));

            fn trampoline(
                _: *mut rsmlui_sys::context::Context,
                key: KeyCode,
                key_modifier: KeyModifier,
                native_dp_ratio: f32,
                priority: bool,
            ) -> bool {
                let result = BACKEND_EVENTS_CALLBACK.with(|callback| {
                    let mut cb = callback.borrow_mut();

                    if let Some(cb) = cb.as_mut() {
                        return cb(key, key_modifier, native_dp_ratio, priority);
                    }

                    true
                });

                result
            }

            while !app.should_exit() {
                if let Some(context) = app.get_context() {
                    let running = unsafe {
                        rsmlui_sys::backend::process_events(
                            context.raw(),
                            trampoline,
                            self.power_save,
                        )
                    };

                    if !running {
                        tx.send(WindowEvent::ExitRequested)
                            .map_err(|_| RsmlUiError::EventSendFailed)?;
                    }
                }

                while let Ok(event) = rx.try_recv() {
                    app.handle_event(event)?;
                }

                app.request_render()?;
            }

            Ok(())
        }
    }
}

pub(crate) use driver::MonolithicBackendDriver;
