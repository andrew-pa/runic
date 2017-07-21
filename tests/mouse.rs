extern crate runic;

use runic::*;

struct TestApp {
    mouse_loc: Point,
    mouse_button: Option<MouseButton>,
}

impl App for TestApp {
    fn paint(&self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.fill_rect(Rect::pnwh(self.mouse_loc, 32.0, 32.0), self.mouse_button.map_or(Color::rgb(0.2, 0.2, 0.2), |v| match v {
            MouseButton::Left => Color::rgb(0.8, 0.2, 0.0),
            MouseButton::Middle => Color::rgb(0.2, 0.8, 0.0),
            MouseButton::Right => Color::rgb(0.2, 0.0, 0.8)
        }));
    }

    fn event(&mut self, e: Event) {
        match e {
            Event::MouseMove(p, mb) => {
                self.mouse_loc = p;
                self.mouse_button = mb;
            },
            Event::MouseUp(_,_) => { self.mouse_button = None; },
            Event::MouseDown(p,mb) => { self.mouse_loc = p; self.mouse_button = Some(mb); },
            _ => {}
       }
    }
}

#[test]
fn mouse() {
    let mut app = TestApp { mouse_loc: Point{x: 256.0, y: 256.0}, mouse_button: None };
    let mut window = Window::new("Mouse Test", 512, 512, &mut app).expect("create window!");
    window.show();
}
