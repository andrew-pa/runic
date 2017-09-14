extern crate runic;
extern crate winit;

use runic::*;
use winit::*;

struct TestApp {
    font: Font, layout: TextLayout
}

impl TestApp {
    fn new(rx: &mut RenderContext) -> TestApp {
        let mut font = rx.new_font("Arial", 32.0, FontWeight::Regular, FontStyle::Normal).expect("load font");
        let layout = rx.new_text_layout("Hello, 😌Text Layouts!😄", &font, 512.0, 512.0).expect("create text layout");
        TestApp {
            font, layout
        }
    }
}

impl App for TestApp {

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(0.3, 0.6, 0.2));
        rx.draw_text(Rect::xywh(8.0, 8.0, 512.0, 512.0), "Hello, draw_text!", &self.font);
        rx.set_color(Color::rgb(0.6, 0.2, 0.3));
        rx.draw_text_layout(Point::xy(8.0, 80.0), &self.layout);
        rx.set_color(Color::rgb(0.9, 0.1, 0.2));
        rx.stroke_rect(self.layout.bounds().offset(Point::xy(8.0, 80.0)), 2.0);
        let cb = self.layout.char_bounds(8);
        rx.set_color(Color::rgb(0.0, 0.6, 0.0));
        rx.stroke_rect(cb.offset(Point::xy(8.0, 80.0)), 2.0);
    }

    fn event(&mut self, e: Event) -> bool {
        false
    }
}

#[test]
fn text() {
    runic::init();
    let mut evl = EventsLoop::new();
    let mut window = WindowBuilder::new().with_dimensions(512, 521).with_title("Text!").build(&evl).expect("create window!");
    let mut rx = RenderContext::new(&mut window).expect("create render context!");
    let mut app = TestApp::new(&mut rx);
    app.run(&mut rx, &mut evl);
}
