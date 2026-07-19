use rsmlui_backends::interfaces::RenderDriver;
use rsmlui_core::core::context::Context;
use rsmlui_core::types::input::{KeyCode, KeyModifier};
use winit::application::ApplicationHandler;
use winit::event::{Modifiers, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey::Code;
use winit::keyboard::{KeyCode as WinitKeyCode, ModifiersState, PhysicalKey};
use winit::window::WindowId;

pub trait RsmlUiAppProvider {
    type Renderer: RenderDriver;

    fn context(&self, id: WindowId) -> Option<&'_ Context>;
    fn renderer(&mut self, id: WindowId) -> Option<&mut Self::Renderer>;
}

pub struct RsmlUiApp<A> {
    inner: A,
    modifiers: Modifiers,
}

impl<A> RsmlUiApp<A> {
    pub fn new(inner: A) -> Self {
        Self {
            inner,
            modifiers: Default::default(),
        }
    }
}

impl<A> ApplicationHandler for RsmlUiApp<A>
where
    A: ApplicationHandler + RsmlUiAppProvider,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.inner.resumed(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match &event {
            WindowEvent::Resized(size) => {
                if let Some(context) = self.inner.context(id) {
                    context.set_dimensions((size.width, size.height));
                }

                if let Some(renderer) = self.inner.renderer(id) {
                    renderer.set_viewport((size.width, size.height));
                }
            },

            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.inner.renderer(id) {
                    renderer.clear();
                    renderer.begin_frame();
                }

                if let Some(context) = self.inner.context(id) {
                    // TODO: should we unwrap?
                    context.update().unwrap();
                    // TODO: should we unwrap?
                    context.render().unwrap();
                }

                if let Some(renderer) = self.inner.renderer(id) {
                    renderer.end_frame();
                }
            },

            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = *modifiers;
            },

            _ => {},
        }

        if let Some(ctx) = self.inner.context(id) {
            forward_window_event(ctx, &event, self.modifiers.state());
        }

        self.inner.window_event(event_loop, id, event);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.inner.suspended(event_loop);
    }
}

fn forward_window_event(context: &Context, event: &WindowEvent, modifiers: ModifiersState) -> bool {
    let mods = translate_modifiers(modifiers);

    match event {
        WindowEvent::CursorMoved { position, .. } => {
            // context.process_mouse_move(position.x as f32, position.y as f32, mods);
            false
        },
        WindowEvent::MouseInput { state, button, .. } => {
            // let btn = translate_mouse_button(*button);
            // match state {
            //     ElementState::Pressed => context.process_mouse_button_down(btn, mods),
            //     ElementState::Released => context.process_mouse_button_up(btn, mods),
            // }

            false
        },
        WindowEvent::MouseWheel { delta, .. } => {
            // let lines = match delta {
            //     MouseScrollDelta::LineDelta(_, y) => *y,
            //     MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 20.0,
            // };
            // context.process_mouse_wheel(lines, mods)
            false
        },
        WindowEvent::KeyboardInput { event, .. } => {
            // let key = translate_key(event.physical_key);
            // if event.state == ElementState::Pressed {
            //     if let Some(text) = &event.text {
            //         for ch in text.chars() {
            //             context.process_text_input(ch);
            //         }
            //     }
            //     context.process_key_down(key, mods)
            // } else {
            //     context.process_key_up(key, mods)
            // }
            false
        },
        _ => false,
    }
}

fn translate_modifiers(modifier: ModifiersState) -> KeyModifier {
    let mut out = KeyModifier::empty();

    if modifier.shift_key() {
        out |= KeyModifier::SHIFT;
    }
    if modifier.control_key() {
        out |= KeyModifier::CTRL;
    }
    if modifier.alt_key() {
        out |= KeyModifier::ALT;
    }

    out
}

fn translate_mouse_button(button: MouseButton) -> i32 {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Back => 3,
        MouseButton::Forward => 4,
        MouseButton::Other(n) => n as i32,
        _ => 0,
    }
}

fn translate_key(key: PhysicalKey) -> KeyCode {
    match key {
        Code(WinitKeyCode::Space) => KeyCode::Space,
        Code(WinitKeyCode::ArrowLeft) => KeyCode::Left,
        Code(WinitKeyCode::ArrowRight) => KeyCode::Right,
        Code(WinitKeyCode::ArrowUp) => KeyCode::Up,
        Code(WinitKeyCode::ArrowDown) => KeyCode::Down,
        Code(WinitKeyCode::Backspace) => KeyCode::Back,
        Code(WinitKeyCode::Delete) => KeyCode::Delete,
        Code(WinitKeyCode::Enter) => KeyCode::Return,
        Code(WinitKeyCode::Tab) => KeyCode::Tab,
        Code(WinitKeyCode::Escape) => KeyCode::Execute,
        Code(WinitKeyCode::Home) => KeyCode::Home,
        Code(WinitKeyCode::End) => KeyCode::End,
        // TODO: rest of the keys
        _ => KeyCode::Unknown,
    }
}
