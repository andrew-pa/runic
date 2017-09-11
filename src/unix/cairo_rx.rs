extern crate cairo;
use self::cairo::Context;

use *;

pub struct RenderContext {
    cx: Context
}

pub struct Font;

impl Font {
    pub fn new(rx: &mut RenderContext, name: &str, size: f32, weight: FontWeight, style: FontStyle) -> Result<Font, Box<Error>> {
        Ok(Font)
    }
}

pub struct TextLayout;

impl TextLayout {
    pub fn new(rx: &mut RenderContext, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<Error>> {
        Ok(TextLayout)
    }

    pub fn bounds(&self) -> Rect {Rect::xywh(0.0,0.0,0.0,0.0)}
    pub fn char_bounds(&self, index: usize) -> Rect {Rect::xywh(0.0,0.0,0.0,0.0)}
}

impl RenderContext {
    pub fn new(cx: Context) -> RenderContext {
        RenderContext { cx }
    }

    fn set_color(&mut self, col: Color) {
        self.cx.set_source_rgba(col.r as f64, col.g as f64, col.b as f64, col.a as f64);
    }

    pub fn clear(&mut self, col: Color) {
        self.set_color(col);
        self.cx.paint();
    }
    pub fn stroke_rect(&mut self, rect: Rect, col: Color, stroke_width: f32) {
        self.set_color(col);
        self.cx.set_line_width(stroke_width as f64);
        self.cx.rectangle(rect.x as f64, rect.y as f64, rect.w as f64, rect.h as f64);
        self.cx.stroke();
    }

    pub fn fill_rect(&mut self, rect: Rect, col: Color) {
        self.set_color(col);
        self.cx.rectangle(rect.x as f64, rect.y as f64, rect.w as f64, rect.h as f64);
        self.cx.fill();
    }

    pub fn draw_line(&mut self, a: Point, b: Point, col: Color, stroke_width: f32) {
        self.set_color(col);
        self.cx.set_line_width(stroke_width as f64);
        self.cx.move_to(a.x as f64, a.y as f64);
        self.cx.line_to(b.x as f64, b.y as f64);
        self.cx.stroke();
    }

    pub fn draw_text(&mut self, rect: Rect, s: &str, col: Color, f: &Font) {}

    pub fn draw_text_layout(&mut self, p: Point, txl: &TextLayout, col: Color) {}

    pub fn translate(&mut self, p: Point) {
        self.cx.translate(p.x as f64, p.y as f64);
    }

    pub fn bounds(&self) -> Rect {
        Rect::xywh(0.0, 0.0, 100.0, 100.0)
    }
}


