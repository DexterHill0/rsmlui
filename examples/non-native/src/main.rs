use rsmlui::core::core::Rml;
use rsmlui::core::font::FontFaceSource;
use rsmlui::interfaces::default::FileDefault;
use rsmlui::monolithic::{BackendOptions, GlfwGl2Backend};

fn main() {
    let file_interface = FileDefault::new();

    let mut backend =
        GlfwGl2Backend::initialize("Demo", (800, 600), BackendOptions::default()).unwrap();

    backend.set_key_down_callback(Box::new(|_, key, _, _, _| {
        println!("key pressed: {key:?}");

        false
    }));

    let rml = Rml::new(backend.backend_handle());

    rml.set_system_interface(Some(backend.get_system_interface().unwrap()));
    rml.set_render_interface(Some(backend.get_render_interface().unwrap()));
    rml.set_file_interface(Some(&file_interface));

    rml.initialise().unwrap();

    rml.load_font_face(FontFaceSource::Path("../assets/Roboto.ttf"), "Roboto")
        .unwrap();

    let context = rml.create_context("main", (800, 600)).unwrap();

    let document = context.load_document("../assets/basic.rml").unwrap();

    document.show();

    backend.run(&context).unwrap();
}
