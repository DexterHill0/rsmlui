use rsmlui::core::core::Rml;
use rsmlui::core::font::FontFaceSource;
use rsmlui::interfaces::default::FileDefault;
use rsmlui::monolithic::BackendOptions;
#[cfg(target_os = "windows")]
use rsmlui::monolithic::Win32Gl2Backend;

#[cfg(not(target_os = "windows"))]
compile_error!("Unsupported OS");

fn main() {
    let mut file_interface = FileDefault::new();

    let mut backend =
        Win32Gl2Backend::initialize("Demo", (800, 600), BackendOptions::default()).unwrap();

    backend.set_key_down_callback(Box::new(|_, key, _, _, _| {
        println!("key pressed: {key:?}");

        false
    }));

    let rml = Rml::new(backend.backend_handle());

    // the borrowed interfaces hold a shared borrow of `backend` until they drop, so these
    // are scoped to release that as `backend.run` needs `&mut backend`
    {
        let mut system = backend.get_system_interface().unwrap();

        rml.set_system_interface(Some(&mut system));
    }

    {
        let mut renderer = backend.get_render_interface().unwrap();

        rml.set_render_interface(Some(&mut renderer));
    }

    rml.set_file_interface(Some(&mut file_interface));

    rml.initialise().unwrap();

    rml.load_font_face(FontFaceSource::Path("../assets/Roboto.ttf"), "Roboto")
        .unwrap();

    let context = rml.create_context("main", (800, 600)).unwrap();

    let document = context.load_document("../assets/basic.rml").unwrap();

    document.show();

    backend.run(&context).unwrap();
}
