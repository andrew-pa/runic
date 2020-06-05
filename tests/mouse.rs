use runic::*;

struct TestApp {
    mouse_loc: Point,
    mouse_button: Option<MouseButton>,
}

impl App for TestApp {
    fn init(_: &mut RenderContext) -> Self {
        TestApp { mouse_loc: Point::xy(0.0,0.0), mouse_button: None }
    }

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

    fn event(&mut self, e: Event, elf: &mut ControlFlowOpts, should_redraw: &mut bool) {
        match e {
            Event::CloseRequested => *elf = ControlFlowOpts::Exit,
            Event::CursorMoved { position: dpi::PhysicalPosition { x,y }, .. } => {
                println!("{:?}", e);
                self.mouse_loc = Point::xy(x as f32,y as f32);
                *should_redraw = true;
            },
            Event::MouseInput { state, button, .. } => {
                self.mouse_button = match state {
                    ElementState::Pressed => Some(button),
                    _ => None
                };
                *should_redraw = true;
            },
            _ => {}
        } 
    }
}

#[test]
fn mouse() {
    runic::start::<TestApp>(WindowOptions::new().with_title("Mouse Test"));
}
