use rsmlui::backends::win32_gl2::BackendWin32Gl2;
use rsmlui::core::core::RsmlUi;
use rsmlui::errors::RsmlUiError;
use rsmlui::glam::IVec2;
use rsmlui::interfaces::backend::{Backend, BackendOptions};

fn main() -> Result<(), RsmlUiError> {
    let dimensions = IVec2::new(800, 600);

    let backend = BackendWin32Gl2::initialize_with_options(
        "rsmlui basic demo",
        dimensions,
        BackendOptions { allow_resize: true },
    )?;

    let mut app = RsmlUi::initialise()?;

    app.use_backend(backend);

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
