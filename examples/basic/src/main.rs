use rsmlui::core::core::Rml;
use rsmlui::monolithic::{Win32Gl2Backend, Win32Gl2BackendOptions};

fn main() {
    let mut backend =
        Win32Gl2Backend::initialize("Demo", (800, 600), Win32Gl2BackendOptions::default()).unwrap();

    backend.set_key_down_callback(Box::new(|_, key, _, _, _| {
        println!("key pressed: {key:?}");

        false
    }));

    let rml = Rml::new(backend.backend_handle());

    rml.set_system_interface(Some(backend.get_system_interface().unwrap()));
    rml.set_render_interface(Some(backend.get_render_interface().unwrap()));

    rml.initialise().unwrap();

    rml.load_font_face("../assets/Roboto.ttf").unwrap();

    let context = rml.create_context("main", (800, 600)).unwrap();

    let document = context.load_document("../assets/basic.rml").unwrap();

    document.show();

    backend.run(&context).unwrap();
}
