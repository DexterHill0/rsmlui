use rsmlui::backends::s_win32_r_gl2::BackendWin32Gl2;
use rsmlui::core::app::ApplicationHandler;
use rsmlui::core::backend::BackendOptions;
use rsmlui::core::context::Context;
use rsmlui::core::core::{ActiveApp, RsmlUi, RsmlUiUninitialized};
use rsmlui::core::element_document::ElementDocument;
use rsmlui::core::events::{KeyboardEvent, WindowEvent};
use rsmlui::errors::RsmlUiError;
use rsmlui::glam::IVec2;

const DIMENSIONS: IVec2 = IVec2::new(800, 600);

struct App {
    context: Option<Context>,
    document: Option<ElementDocument>,
}

impl ApplicationHandler for App {
    fn starting(&mut self, app: &mut ActiveApp) -> Result<(), RsmlUiError> {
        app.load_font_face("../assets/Roboto.ttf")?;

        let context = app.create_context("main", DIMENSIONS)?;
        let document = context.load_document("../assets/basic.rml")?;

        document.show();

        self.document = Some(document);
        self.context = Some(context);

        Ok(())
    }

    fn event(&mut self, event: WindowEvent, app: &mut ActiveApp) -> Result<(), RsmlUiError> {
        match event {
            WindowEvent::ExitRequested => app.exit(),
            WindowEvent::UpdateRequested => {
                if let Some(context) = self.context.as_ref() {
                    context.update()?;
                }
            },
            WindowEvent::RenderRequested(..) => {
                if let Some(context) = self.context.as_ref() {
                    context.render()?;
                }
            },
            WindowEvent::KeyboardEvent(event) => match event {
                KeyboardEvent::KeyPressed {
                    code,
                    modifier: modifiers,
                    native_dp_ratio,
                    fallback,
                } => println!(
                    "key: {code:?}, modifiers: {modifiers:?}, dp ratio: {native_dp_ratio}, fallback {fallback}"
                ),
                _ => {},
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
    let backend = BackendWin32Gl2::new(
        "rsmlui basic demo",
        DIMENSIONS,
        BackendOptions {
            allow_resize: true,
            power_save: false,
        },
    );

    let mut rsmlui = RsmlUiUninitialized::new_with_monolithic_backend(backend).initialize()?;

    let app = App {
        context: None,
        document: None,
    };

    rsmlui.run_app(app)?;

    Ok(())
}
