extern crate runic;
extern crate winit;

use runic::*;
use winit::*;

struct TestApp {
    font: Font,
    layout: TextLayout,
    layout2: TextLayout,
}

impl TestApp {
    fn new(rx: &mut RenderContext) -> TestApp {
        let mut font = rx.new_font("Arial", 40.0, FontWeight::Regular, FontStyle::Normal).expect("load font");
        let layout = rx.new_text_layout("The quick brown fox jumps over the lazy dog!", &font, 1000.0, 128.0).expect("create text layout");
        layout.color_range(rx, 0..3, Color::rgb(0.3, 0.3, 0.3));
        layout.color_range(rx, 31..34, Color::rgb(0.3, 0.3, 0.3));
        layout.color_range(rx, 10..15, Color::rgb(0.4, 0.2, 0.0));
        layout.color_range(rx, 35..39, Color::rgb(0.0, 0.2, 0.6));
        layout.color_range(rx, 26..30, Color::rgb(0.1, 0.6, 0.1));
        layout.style_range(4..9, FontStyle::Italic);
        layout.weight_range(20..25, FontWeight::Bold);
        layout.weight_range(0..3, FontWeight::Light);
        layout.weight_range(31..34, FontWeight::Light);
        layout.underline_range(26..30, true);
        let layout2 = rx.new_text_layout("This layout has dynamic styling!", &font, 1000.0, 128.0).expect("create text layout");
        TestApp {
            font, layout, layout2
        }
    }
}

impl App for TestApp {

    fn paint(&mut self, rx: &mut RenderContext) {
        rx.clear(Color::rgb(0.1, 0.1, 0.12));
        rx.set_color(Color::rgb(0.88, 0.88, 0.80));
        rx.draw_text_layout(Point::xy(8.0, 8.0), &self.layout);
        let b = self.layout.bounds();
        rx.draw_text_layout(Point::xy(8.0, 16.0+b.h), &self.layout2);
    }

    fn event(&mut self, e: Event) -> bool {
        match e {
            Event::WindowEvent { event: e, .. } => match e {
                WindowEvent::CursorMoved { position: (x,y), .. } => {
                    let b = self.layout.bounds();
                    self.layout2.underline_range(0..32, false);
                    if let Some((i, r)) = self.layout2.hit_test(Point::xy(x as f32 - 8.0, y as f32 - (16.0+b.h)))  {
                        self.layout2.underline_range(0..(i as u32 + 1), true);
                        self.layout2.style_range((i as u32)..(i as u32 + 1), FontStyle::Italic);
                    }
                }
                _=>{}
            },
            _=>{}
        }
        false
    }
}

#[test]
fn range_styles() {
    runic::init();
    let mut evl = EventsLoop::new();
    let mut window = WindowBuilder::new().with_dimensions(1000, 256).with_title("Styled Ranges").build(&evl).expect("create window!");
    let mut rx = RenderContext::new(&mut window).expect("create render context!");
    let mut app = TestApp::new(&mut rx);
    app.run(&mut rx, &mut evl);
}
