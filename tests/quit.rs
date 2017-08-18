extern crate runic;

use runic::*;

struct TestApp {
}

impl App for TestApp {
    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.fill_rect(Rect::xywh(64.0, 64.0, 100.0, 100.0), Color::rgb(1.0, 0.05, 0.4));
    }

    fn event(&mut self, e: Event, win: WindowRef) {
        match e {
            Event::Key(KeyCode::Escape, _) => win.quit(),
            _ => ()
        }
    }
}

#[test]
fn basic() {
    let mut window = Window::new("Quit Test: Press Escape to Quit", 512, 512, |_| TestApp{}).expect("create window!");
    window.show();
}
