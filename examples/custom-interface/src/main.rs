use std::time::Instant;

use rsmlui::backends::win32_gl2::BackendWin32Gl2;
use rsmlui::core::core::RsmlUi;
use rsmlui::core::log::LogLevel;
use rsmlui::errors::RsmlUiError;
use rsmlui::glam::IVec2;
use rsmlui::interfaces::backend::{Backend, BackendOptions};
use rsmlui::interfaces::system::SystemInterfaceBehaviour;
use rsmlui::interfaces::{InterfaceInstancer, InterfaceState};

struct CustomSystemInterface {
    start: Instant,
}

impl SystemInterfaceBehaviour for InterfaceState<CustomSystemInterface> {
    fn get_elapsed_time(&mut self) -> f64 {
        (std::time::Instant::now() - self.start).as_secs_f64()
    }

    fn translate_string(&mut self, input: &str) -> String {
        return format!("{input}+translated");
    }

    fn log_message(&mut self, level: LogLevel, message: &str) -> bool {
        match level {
            LogLevel::LT_ALWAYS => println!("[ALW]: {message}"),
            LogLevel::LT_ASSERT => {
                eprintln!("[FTL]: {message}");
                panic!("assert triggered");
            },
            LogLevel::LT_ERROR => eprintln!("[ERR]: {message}"),
            LogLevel::LT_WARNING => println!("[WRN]: {message}"),
            LogLevel::LT_INFO => println!("[INF]: {message}"),
            LogLevel::LT_DEBUG => println!("[DEB]: {message}"),
            _ => {},
        }

        return true;
    }
}

fn main() -> Result<(), RsmlUiError> {
    let dimensions = IVec2::new(800, 600);

    let backend = BackendWin32Gl2::initialize_with_options(
        "rsmlui basic demo",
        dimensions,
        BackendOptions { allow_resize: true },
    )?;

    let mut app = RsmlUi::initialise()?;

    app.use_backend(backend);

    let custom_system_interface = CustomSystemInterface {
        start: Instant::now(),
    };

    todo!();

    // backend.set_event_callback(|event| {
    //     println!("processing {:?}", event.key);

    //     return true;
    // });

    app.load_font_face("../assets/Roboto.ttf")?;

    let context = app
        .create_context("main", dimensions)
        .expect("failed to create context");

    let document = context
        .load_document("../assets/basic.rml")
        .expect("failed to create document");

    document.show();

    let mut running = true;

    while running {
        // running = backend.process_events(&mut context, ProcessEventsOptions::default());

        context.update()?;

        app.begin_frame();

        context.render()?;

        app.present_frame();
    }

    Ok(())
}
