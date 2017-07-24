extern crate runic;

use runic::*;

struct TestApp {
}

impl App for TestApp {
    fn paint(&self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.stroke_rect(Rect::xywh(8.0, 8.0, 100.0, 100.0), Color::rgb(0.4, 0.05, 1.0), 8.0);
        rx.fill_rect(Rect::xywh(116.0, 8.0, 100.0, 100.0), Color::rgb(0.4, 0.05, 1.0));
        rx.draw_line(Point::xy(16.0, 16.0), Point::xy(94.0, 94.0), Color::rgb(0.0, 0.0, 0.6), 3.0);
    }

    fn event(&mut self, e: Event) {
    }
}

#[test]
fn shapes() {
    let mut window = Window::new("Shapes!", 512, 512, |_| TestApp{}).expect("create window!");
    window.show();
}
