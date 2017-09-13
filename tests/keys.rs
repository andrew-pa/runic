extern crate runic;
extern crate winit;

use runic::*;
use winit::*;


struct TestApp {
    font: Font, last_kbi: Option<KeyboardInput>
}

impl TestApp {
    fn new(rx: &mut RenderContext) -> TestApp {
        let font = rx.new_font("Arial", 32.0, FontWeight::Regular, FontStyle::Normal).expect("load font");
        TestApp {
            font, last_kbi: None
        }
    }
}

impl App for TestApp {

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(0.2, 0.2, 0.2));
        rx.draw_text(Rect::xywh(8.0, 8.0, 512.0, 512.0), &format!("{:?}", self.last_kbi), &self.font);
    }

    fn event(&mut self, e: Event) -> bool{
        match e {
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: kbi, .. }, .. } => {
                self.last_kbi = Some(kbi);
            }
            _ => {}
        }
        false
    }
}

#[test]
fn keys() {
    let mut evl = EventsLoop::new();
    let mut window = WindowBuilder::new().with_dimensions(512, 521).with_title("Keyboard Test").build(&evl).expect("create window!");
    let mut rx = RenderContext::new(&window).expect("create render context!");
    let mut app = TestApp::new(&mut rx);
    app.run(&mut rx, &mut evl);
}
