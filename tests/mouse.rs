extern crate runic;
extern crate winit;

use runic::*;
use winit::*;

struct TestApp {
    mouse_loc: Point,
    mouse_button: Option<MouseButton>,
}

impl App for TestApp {
    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(self.mouse_button.map_or(Color::rgb(0.2, 0.2, 0.2), |v| match v {
            MouseButton::Left => Color::rgb(0.8, 0.2, 0.0),
            MouseButton::Middle => Color::rgb(0.2, 0.8, 0.0),
            MouseButton::Right => Color::rgb(0.2, 0.0, 0.8),
            _ => Color::rgb(0.0, 0.0, 0.0)
        }));
        rx.fill_rect(Rect::pnwh(self.mouse_loc, 32.0, 32.0));
    }

    fn event(&mut self, e: Event) -> bool {
        match e {
            Event::WindowEvent { event: e, .. } => match e {
                WindowEvent::CursorMoved { position: (x,y), .. } => {
                    self.mouse_loc = Point::xy(x as f32,y as f32);
                },
                WindowEvent::MouseInput { state, button, .. } => {
                    self.mouse_button = match state {
                        ElementState::Pressed => Some(button),
                        _ => None
                    }
                },
                _ => {}
            }
            _ => {}
        }
        false
    }
}

#[test]
fn mouse() {
    runic::init();
    let mut evl = EventsLoop::new();
    let mut window = WindowBuilder::new().with_dimensions(512, 521).with_title("Mouse Test").build(&evl).expect("create window!");
    let mut rx = RenderContext::new(&mut window).expect("create render context!");
    let mut app = TestApp{mouse_loc: Point::xy(0.0,0.0), mouse_button: None};
    app.run(&mut rx, &mut evl);
}
