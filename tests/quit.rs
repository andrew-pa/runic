extern crate runic;
extern crate winit;

use runic::*;

struct TestApp {
}

impl App for TestApp {
    fn init(_ : &mut RenderContext) -> Self { TestApp { } }

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(1.0, 0.05, 0.4));
        rx.fill_rect(Rect::xywh(64.0, 64.0, 100.0, 100.0));
    }

    fn event(&mut self, e: Event) -> bool {
        match e {
            Event::CloseRequested => true,
            Event::KeyboardInput { input, .. } => {
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => true,
                    _ => false
                }
            },
            _ => false
        }
    }
}

#[test]
fn quit() {
    runic::start::<TestApp>(WindowOptions::new().with_title("Quit"));
}
