
extern crate runic;

use runic::*;

struct TestApp {
}

impl App for TestApp {
    fn paint(&self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
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
