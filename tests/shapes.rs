extern crate runic;
extern crate winit;

use runic::*;
use winit::*;

struct TestApp {
}

impl App for TestApp {
    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(0.4, 0.05, 1.0));
        rx.stroke_rect(Rect::xywh(8.0, 8.0, 100.0, 100.0), 8.0);
        rx.set_color(Color::rgb(0.4, 0.05, 1.0));
        rx.fill_rect(Rect::xywh(116.0, 8.0, 100.0, 100.0));
        rx.set_color(Color::rgb(0.0, 0.0, 0.6));
        rx.draw_line(Point::xy(16.0, 16.0), Point::xy(94.0, 94.0), 3.0);
        let b = rx.bounds();
        rx.stroke_rect(b, 4.0);
    }

    fn event(&mut self, e: Event) -> bool {
        false
    }
}

#[test]
fn shapes() {
    runic::init();
    let mut evl = EventsLoop::new();
    let mut window = WindowBuilder::new().with_dimensions(512, 512).with_title("Shapes!").build(&evl).expect("create window!");
    let mut rx = RenderContext::new(&mut window).expect("create render context!");
    let mut app = TestApp{};
    app.run(&mut rx, &mut evl);
}
