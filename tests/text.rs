extern crate runic;

use runic::*;

struct TestApp {
    font: Option<Font>, layout: Option<TextLayout>
}

impl App for TestApp {
    fn init(&mut self, rx: &mut RenderContext) {
        self.font = Some(Font::new(rx, String::from("Arial"), 32.0, FontWeight::Regular, FontStyle::Normal).expect("load font"));
        self.layout = Some(TextLayout::new(rx, "Hello, Text Layouts!😄", self.font.as_ref().unwrap(), 512.0, 512.0).expect("create text layout")); 
    }

    fn paint(&self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.draw_text(Rect::xywh(8.0, 8.0, 512.0, 512.0), "Hello, draw_text!", Color::rgb(0.3, 0.6, 0.2), self.font.as_ref().unwrap());
        rx.draw_text_layout(Point::xy(8.0, 80.0), self.layout.as_ref().unwrap(), Color::rgb(0.6, 0.2, 0.3));
    }

    fn event(&mut self, e: Event) {
    }
}

#[test]
fn text() {
    let mut app = TestApp { font: None, layout: None };
    let mut window = Window::new("Text Render Test", 512, 512, &mut app).expect("create window!");
    window.show();
}
