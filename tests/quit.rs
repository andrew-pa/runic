extern crate runic;
extern crate winit;

use runic::*;
use winit::*;

struct TestApp {
}

impl App for TestApp {
    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(1.0, 0.05, 0.4));
        rx.fill_rect(Rect::xywh(64.0, 64.0, 100.0, 100.0));
    }

    fn event(&mut self, e: Event) -> bool {
        match e {
            Event::WindowEvent { event: e, .. } => {
                match e {
                    WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Escape) => true,
                            _ => false
                        }
                    },
                    _ => false
                }
            },
            _ => false
        }
    }
}

#[test]
fn quit() {
    runic::init();
    let mut evl = EventsLoop::new();
    let mut window = WindowBuilder::new().with_dimensions(512, 521).with_title("Quiting!").build(&evl).expect("create window!");
    let mut rx = RenderContext::new(&mut window).expect("create render context!");
    let mut app = TestApp{};
    app.run(&mut rx, &mut evl);
}
