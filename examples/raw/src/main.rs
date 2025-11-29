use rsmlui_sys::{backend, context, core, element_document};

fn main() {
    let success = backend::initialize("rsmlui-sys basic demo".into(), 800, 600, true);

    if !success {
        panic!("failed to initialize backend!");
    }

    unsafe {
        core::set_system_interface(backend::get_system_interface());
        core::set_render_interface(backend::get_render_interface());
    }

    let success = core::initialise();

    if !success {
        panic!("failed to initialize rmlui!");
    }

    let context = core::create_context("main".into(), 800, 600);

    if context.is_null() {
        core::shutdown();
        backend::shutdown();

        panic!("failed to create context!");
    }

    let document = unsafe {
        context::context_load_document(
            context,
            "D:/Dexter/Documents/Projects/rsmlui/examples/raw/src/test.rml".into(),
        )
    };

    if document.is_null() {
        core::shutdown();
        backend::shutdown();

        panic!("failed to create document!");
    }

    unsafe { element_document::element_document_show(document) };

    // TODO: shutdown

    let mut running = true;

    while running {
        running = unsafe {
            backend::process_events(
                context,
                |_, _, _, _, _| {
                    println!("processing");
                    return true;
                },
                false,
            )
        };

        let success = unsafe { context::context_update(context) };

        if !success {
            panic!("context update failed!");
        }

        backend::begin_frame();
        let success = unsafe { context::context_render(context) };
        if !success {
            panic!("context render failed!");
        }

        backend::present_frame();
    }

    core::shutdown();
    backend::shutdown();
}
