extern crate runic;
extern crate winit;

use runic::*;


struct TestApp {
    font: Font, last_kbi: Option<KeyboardInput>
}

impl App for TestApp {
    fn init(rx: &mut RenderContext) -> TestApp {
        let font = rx.new_font("Arial", 32.0, FontWeight::Regular, FontStyle::Normal).expect("load font");
        TestApp {
            font, last_kbi: None
        }
    }

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(0.2, 0.2, 0.2));
        rx.draw_text(Rect::xywh(8.0, 8.0, 512.0, 512.0), &format!("{:?}", self.last_kbi), &self.font);
    }

    fn event(&mut self, e: Event) -> bool{
        match e {
            Event::CloseRequested => true,
            Event::KeyboardInput { input: kbi, .. } => {
                self.last_kbi = Some(kbi);
                false
            }
            _ => {false}
        }
    }
}

#[test]
fn keys() {
  runic::start::<TestApp>(WindowOptions::new().with_title("Keyboard Test")) 
}
