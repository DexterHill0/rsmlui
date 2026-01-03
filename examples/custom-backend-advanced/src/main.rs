// use std::time::Instant;

// use rsmlui::backends::s_win32_r_gl2::BackendWin32Gl2;
// use rsmlui::core::app::ApplicationHandler;
// use rsmlui::core::backend::{Backend, BackendOptions};
// use rsmlui::core::context::Context;
// use rsmlui::core::core::{ActiveApp, RsmlUiUninitialized};
// use rsmlui::core::element_document::ElementDocument;
// use rsmlui::core::events::{KeyboardEvent, WindowEvent};
// use rsmlui::core::log::LogLevel;
// use rsmlui::errors::RsmlUiError;
// use rsmlui::glam::IVec2;
// use rsmlui::interfaces::InterfaceState;
// use rsmlui::interfaces::system::SystemInterface;
// use rsmlui::window::winit::{WinitWindow, WinitWindowOptions};

// const DIMENSIONS: IVec2 = IVec2::new(800, 600);

// struct CustomSystemInterface {
//     start: Instant,
// }

// impl SystemInterface for InterfaceState<CustomSystemInterface> {
//     fn get_elapsed_time(&mut self) -> f64 {
//         (std::time::Instant::now() - self.start).as_secs_f64()
//     }

//     fn translate_string(&mut self, input: &str) -> String {
//         return format!("{input}+translated");
//     }

//     fn log_message(&mut self, level: LogLevel, message: &str) -> bool {
//         match level {
//             LogLevel::LT_ALWAYS => println!("[ALW]: {message}"),
//             LogLevel::LT_ASSERT => {
//                 eprintln!("[FTL]: {message}");
//                 panic!("assert triggered");
//             },
//             LogLevel::LT_ERROR => eprintln!("[ERR]: {message}"),
//             LogLevel::LT_WARNING => println!("[WRN]: {message}"),
//             LogLevel::LT_INFO => println!("[INF]: {message}"),
//             LogLevel::LT_DEBUG => println!("[DEB]: {message}"),
//             _ => {},
//         }

//         return true;
//     }
// }

// struct App {
//     context: Option<Context>,
//     document: Option<ElementDocument>,
// }

// impl ApplicationHandler for App {
//     fn starting(&mut self, app: &mut ActiveApp) -> Result<(), RsmlUiError> {
//         app.load_font_face("../assets/Roboto.ttf")?;

//         let context = app.create_context("main", DIMENSIONS)?;
//         let document = context.load_document("../assets/basic.rml")?;

//         document.show();

//         self.document = Some(document);
//         self.context = Some(context);

//         Ok(())
//     }

//     fn event(&mut self, event: WindowEvent, app: &mut ActiveApp) -> Result<(), RsmlUiError> {
//         match event {
//             WindowEvent::ExitRequested => app.exit(),
//             WindowEvent::UpdateRequested => {
//                 if let Some(context) = self.context.as_ref() {
//                     context.update()?;
//                 }
//             },
//             WindowEvent::RenderRequested(..) => {
//                 if let Some(context) = self.context.as_ref() {
//                     context.render()?;
//                 }
//             },
//             WindowEvent::KeyboardEvent(event) => match event {
//                 KeyboardEvent::KeyPressed {
//                     code,
//                     modifier: modifiers,
//                     native_dp_ratio,
//                     fallback,
//                 } => println!(
//                     "key: {code:?}, modifiers: {modifiers:?}, dp ratio: {native_dp_ratio}, fallback {fallback}"
//                 ),
//                 _ => {},
//             },
//             _ => {},
//         }

//         Ok(())
//     }

//     fn get_context(&mut self) -> Option<&mut Context> {
//         self.context.as_mut()
//     }
// }

// fn main() -> Result<(), RsmlUiError> {
//     let custom_backend = Backend::new_with_window(WinitWindow::new_with_options(
//         rsmlui::window::winit::ControlFlow::Poll,
//         WinitWindowOptions::default(),
//     )?)
//     .with_system_uninstanced(CustomSystemInterface {
//         start: Instant::now(),
//     });

//     let mut rsmlui = RsmlUiUninitialized::new_with_custom_backend(custom_backend).initialize()?;

//     let app = App {
//         context: None,
//         document: None,
//     };

//     rsmlui.run_app(app)?;

//     Ok(())
// }

fn main() {}
