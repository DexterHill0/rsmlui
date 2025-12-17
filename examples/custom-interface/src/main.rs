use std::time::Instant;

use rsmlui::backends::win32_gl2::BackendWin32Gl2;
use rsmlui::core::context::Context;
use rsmlui::core::core::{ActiveApp, RsmlUi, RsmlUiApp};
use rsmlui::core::element_document::ElementDocument;
use rsmlui::core::events::WindowEvent;
use rsmlui::core::log::LogLevel;
use rsmlui::errors::RsmlUiError;
use rsmlui::glam::IVec2;
use rsmlui::interfaces::InterfaceState;
use rsmlui::interfaces::backend::{Backend, BackendOptions};
use rsmlui::interfaces::system::SystemInterfaceBehaviour;

const DIMENSIONS: IVec2 = IVec2::new(800, 600);

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

struct App {
    context: Option<Context>,
    document: Option<ElementDocument>,
}

impl RsmlUiApp<BackendWin32Gl2> for App {
    fn starting(&mut self, app: &mut ActiveApp<BackendWin32Gl2>) -> Result<(), RsmlUiError> {
        app.load_font_face("../assets/Roboto.ttf")?;

        let context = app.create_context("main", DIMENSIONS)?;
        let document = context.load_document("../assets/basic.rml")?;

        document.show();

        self.document = Some(document);
        self.context = Some(context);

        Ok(())
    }

    fn event(
        &mut self,
        event: WindowEvent,
        app: &mut ActiveApp<BackendWin32Gl2>,
    ) -> Result<(), RsmlUiError> {
        match event {
            WindowEvent::ExitRequested => app.exit(),
            WindowEvent::RenderRequested => {
                if let Some(context) = self.context.as_ref() {
                    context.update()?;

                    app.begin_frame();

                    context.render()?;

                    app.present_frame();
                }
            },
            _ => {},
        }

        Ok(())
    }

    fn get_context(&mut self) -> Option<&mut Context> {
        self.context.as_mut()
    }
}

fn main() -> Result<(), RsmlUiError> {
    let backend = BackendWin32Gl2::initialize_with_options(
        "rsmlui custom interface demo",
        DIMENSIONS,
        BackendOptions { allow_resize: true },
    )?;

    let mut rsmlui = RsmlUi::new(backend)?;

    let mut app = App {
        context: None,
        document: None,
    };

    rsmlui.run_app(&mut app)?;

    Ok(())
}
