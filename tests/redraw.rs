use runic::*;


struct TestApp {
    font: Font, last_kbi: Option<KeyboardInput>,
    frame_count: usize
}

impl App for TestApp {
    fn init(rx: &mut RenderContext) -> TestApp {
        let font = rx.new_font("Arial", 32.0, FontWeight::Regular, FontStyle::Normal).expect("load font");
        TestApp {
            font, last_kbi: None,
            frame_count: 0
        }
    }

    fn paint(&mut self, rx: &mut RenderContext) {
        self.frame_count += 1;
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(0.2, 0.2, 0.2));
        rx.draw_text(Rect::xywh(8.0, 8.0, 512.0, 512.0), &format!("{:?}", self.frame_count), &self.font);
        rx.draw_text(Rect::xywh(8.0, 80.0, 512.0, 512.0), &format!("{:?}", self.last_kbi), &self.font);
    }

    fn event(&mut self, e: Event, event_loop_flow: &mut ControlFlowOpts, should_redraw: &mut bool) {
        match e {
            Event::CloseRequested => *event_loop_flow = ControlFlowOpts::Exit,
            Event::KeyboardInput { input: kbi, .. } => {
                self.last_kbi = Some(kbi);
                *should_redraw = true;
            }
            _ => {}
        }
    }
}

#[test]
fn redraw() {
    runic::start::<TestApp>(WindowOptions::new().with_title("Redraw Test")) 
}
