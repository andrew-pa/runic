extern crate runic;

use runic::*;

struct TestApp {
}

impl App for TestApp {
    fn paint(&self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.stroke_rect(Rect::xywh(64.0, 64.0, 100.0, 100.0), Color::rgb(0.4, 0.05, 1.0), 8.0);
    }

    fn event(&mut self, e: Event) {
    }
}

#[test]
fn basic() {
    let mut app = TestApp { };
    let mut window = Window::new("Basic Window", 512, 512, &mut app).expect("create window!");
    window.show();
}
