extern crate runic;

use runic::*;

struct TestApp {
    font: Font, layout: TextLayout
}

impl TestApp {
    fn new(rx: &mut RenderContext) -> TestApp {
        let mut font = Font::new(rx, String::from("Arial"), 32.0, FontWeight::Regular, FontStyle::Normal).expect("load font");
        let layout = TextLayout::new(rx, "Hello, Text Layouts!😄", &font, 512.0, 512.0).expect("create text layout");
        TestApp {
            font, layout
        }
    }
}

impl App for TestApp {

    fn paint(&self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.draw_text(Rect::xywh(8.0, 8.0, 512.0, 512.0), "Hello, draw_text!", Color::rgb(0.3, 0.6, 0.2), &self.font);
        rx.draw_text_layout(Point::xy(8.0, 80.0), &self.layout, Color::rgb(0.6, 0.2, 0.3));
        rx.stroke_rect(self.layout.bounds().offset(Point::xy(8.0, 80.0)), Color::rgb(0.9, 0.1, 0.2), 2.0);
    }

    fn event(&mut self, e: Event) {
    }
}

#[test]
fn text() {
    let mut window = Window::new("Text Render Test", 512, 512, TestApp::new).expect("create window!");
    window.show();
}
