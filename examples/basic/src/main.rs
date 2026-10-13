use rsmlui::core::core::Rml;
use rsmlui::core::font::FontFaceSource;
use rsmlui::interfaces::default::FileDefault;
use rsmlui::monolithic::BackendOptions;
#[cfg(target_os = "windows")]
use rsmlui::monolithic::Win32Gl2Backend;

#[cfg(not(target_os = "windows"))]
compile_error!("Unsupported OS");

fn main() {
    let mut backend =
        Win32Gl2Backend::initialize("Demo", (800, 600), BackendOptions::default()).unwrap();

    backend.set_key_down_callback(Box::new(|_, key, _, _, _| {
        println!("key pressed: {key:?}");

        false
    }));

    let mut rml = Rml::new(backend.backend_handle());

    backend.install_interfaces(&mut rml);

    rml.initialise().unwrap();

    rml.load_font_face(FontFaceSource::Path("../assets/Roboto.ttf"), "Roboto")
        .unwrap();

    let context = rml.create_context("main", (800, 600)).unwrap();

    let document = context.load_document("../assets/basic.rml").unwrap();

    document.show();

    backend.run(&context).unwrap();
}
