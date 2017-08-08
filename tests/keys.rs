extern crate runic;

use runic::*;


struct TestApp {
    font: Font, last_char: char, last_key: KeyCode, key_down: bool 
}

impl TestApp {
    fn new(rx: &mut RenderContext) -> TestApp {
        let font = Font::new(rx, "Arial", 32.0, FontWeight::Regular, FontStyle::Normal).expect("load font");
        TestApp {
            font, last_char: ' ', last_key: KeyCode::Unknown, key_down: false 
        }
    }
}

impl App for TestApp {

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.draw_text(Rect::xywh(8.0, 8.0, 512.0, 512.0), &format!("last char: {},\nlast key: {:?},\nkey down?: {}", self.last_char, self.last_key, self.key_down), Color::rgb(0.2, 0.2, 0.2), &self.font);
    }

    fn event(&mut self, e: Event) {
        match e {
            Event::Key(KeyCode::Character(c), kd) => { self.last_char = c; self.key_down = kd; },
            Event::Key(kc, kd) => { self.last_key = kc; self.key_down = kd; },
            _ => {}
        }
    }
}

#[test]
fn keys() {
    let mut window = Window::new("Keyboard Test", 512, 512, TestApp::new).expect("create window!");
    window.show();
}
