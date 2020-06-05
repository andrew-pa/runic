use runic::*;

struct TestApp {
}

impl App for TestApp {
    fn init(_: &mut RenderContext) -> Self { TestApp {} }

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(1.0, 0.4, 0.05));
        rx.set_color(Color::rgb(0.4, 0.05, 1.0));
        rx.stroke_rect(Rect::xywh(8.0, 8.0, 100.0, 100.0), 8.0);
        rx.set_color(Color::rgb(0.4, 0.05, 1.0));
        rx.fill_rect(Rect::xywh(116.0, 8.0, 100.0, 100.0));
        rx.set_color(Color::rgb(0.0, 0.0, 0.6));
        rx.draw_line(Point::xy(16.0, 16.0), Point::xy(94.0, 94.0), 3.0);
        let b = rx.bounds();
        rx.stroke_rect(b, 4.0);
    }

    fn event(&mut self, e: Event, event_loop_flow: &mut ControlFlowOpts, _: &mut bool) {
        if let Event::CloseRequested = e { *event_loop_flow = ControlFlowOpts::Exit; }
    }
}

#[test]
fn shapes() {
    runic::start::<TestApp>(WindowOptions::new());
}

