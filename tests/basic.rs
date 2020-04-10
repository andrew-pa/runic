extern crate runic;
extern crate winit;

use runic::*;

struct TestApp {
}

impl App for TestApp {
    fn init(_: &mut RenderContext) -> Self { TestApp{} }

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(0.4, 0.05, 1.0));
        rx.stroke_rect(Rect::xywh(64.0, 64.0, 100.0, 100.0), 8.0);
    }

    fn event(&mut self, e: Event) -> bool {
        if let Event::CloseRequested = e { true } else { false }
    }
}

#[test]
fn basic() {
    runic::start::<TestApp>(WindowOptions::new().with_title("Basic Window"));
}
