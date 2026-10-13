use rsmlui::core::font::FontFaceSource;
use rsmlui::core::rml::Rml;
use rsmlui::monolithic::{BackendOptions, GlfwGl2Backend};

fn main() {
    let mut backend =
        GlfwGl2Backend::initialize("Demo", (800, 600), BackendOptions::default()).unwrap();

    backend.set_key_down_callback(Box::new(|_, key, _, _, _| {
        println!("key pressed: {key:?}");

        false
    }));

    let mut rml = Rml::new(backend.backend_handle());

    backend.install_interfaces(&mut rml);

    rml.initialise().unwrap();

    rml.load_font_face(FontFaceSource::Path("../assets/Roboto.ttf"), "Roboto")
        .unwrap();

    let mut context = rml.create_context("main", (800, 600)).unwrap();

    let mut document = context.load_document("../assets/basic.rml").unwrap();

    document.show();

    backend.run(&mut context).unwrap();
}
