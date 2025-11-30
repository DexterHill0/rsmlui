use rsmlui::{
    backends::win32_gl2::BackendWin32Gl2,
    core::core::RsmlUi,
    errors::RsmlUiError,
    glam::IVec2,
    interfaces::backend::{Backend, BackendOptions, ProcessEventsOptions},
};

fn main() -> Result<(), RsmlUiError> {
    let dimensions = IVec2::new(800, 600);

    let mut backend = BackendWin32Gl2::initialize_with_options(
        "rsmlui basic demo",
        dimensions,
        BackendOptions { allow_resize: true },
    )?;

    let mut app = RsmlUi::initialise()?;

    backend.set_event_callback(|event| {
        println!("processing!");

        return true;
    });

    app.set_system_interface(
        backend
            .get_system_interface()
            .expect("failed to get system interface"),
    );
    app.set_render_interface(
        backend
            .get_render_interface()
            .expect("failed to get render interface"),
    );

    let mut context = app
        .create_context("main", dimensions)
        .expect("failed to create context");

    let document = context
        .load_document("../../examples/assets/basic.rml")
        .expect("failed to create document");

    document.show();

    let mut running = true;

    while running {
        running = backend.process_events(&mut context, ProcessEventsOptions::default());

        context.update()?;

        backend.begin_frame();

        context.render()?;

        backend.present_frame();
    }

    Ok(())
}
