use *;

use std::error::Error;
use std::os::raw::c_void;
use std::mem::transmute;
use std::rc::Rc;

use cairo_sys::*;
use pango_sys::*;
use pangocairo_sys::*;
use gobject_sys::g_object_unref;

struct PangoFontDesc(*mut PangoFontDescription);

impl Drop for PangoFontDesc {
    fn drop(&mut self) {
        unsafe {
            pango_font_description_free(self.0);
        }
    }
}

pub struct Font(Rc<PangoFontDesc>);

struct PangoLayoutAuto(*mut PangoLayout);

impl Drop for PangoLayoutAuto {
    fn drop(&mut self) {
        unsafe {
            g_object_unref(transmute(self.0));
        }
    }
}

pub struct TextLayout(Rc<PangoLayoutAuto>);

impl TextLayoutExt for TextLayout {
    fn bounds(&self) -> Rect { Rect::xywh(0.0,0.0,0.0,0.0) }
    fn char_bounds(&self, index: usize) -> Rect { Rect::xywh(0.0, 0.0,0.0,0.0) }
}


pub trait CairoSurface {
    fn new(win: &mut winit::Window) -> Result<Self, Box<Error>> where Self: Sized;
    fn start_paint(&mut self);
    fn end_paint(&mut self);
    fn resize(&mut self, w: u32, h: u32);
    fn surface(&self) -> *mut cairo_surface_t;
    fn bounds(&self) -> Rect;
}

pub struct CairoRenderContext<S: CairoSurface> {
    surface: S,
    cx: *mut cairo_t,
    pg: *mut PangoContext,
}

impl<S: CairoSurface> RenderContextExt for CairoRenderContext<S> {
    fn new_font(&self, name: &str, size: f32, weight: FontWeight, style: FontStyle) -> Result<Font, Box<Error>> {
        unsafe {
            let fd = pango_font_description_new();
            pango_font_description_set_family(fd, name.as_ptr() as *const i8);
            pango_font_description_set_size(fd, (size * PANGO_SCALE as f32) as i32);
            pango_font_description_set_weight(fd, match weight {
                FontWeight::Light => PANGO_WEIGHT_LIGHT,
                FontWeight::Regular => PANGO_WEIGHT_MEDIUM,
                FontWeight::Bold => PANGO_WEIGHT_BOLD
            });
            pango_font_description_set_style(fd, match style {
                FontStyle::Normal => PANGO_STYLE_NORMAL,
                FontStyle::Italic => PANGO_STYLE_ITALIC
            });
            Ok(Font(Rc::new(PangoFontDesc(fd))))
        }
    }

    fn new_text_layout(&self, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<Error>> {
        unsafe {
            let ly = pango_layout_new(self.pg);
            pango_layout_set_text(ly, text.as_ptr() as *const i8, text.len() as i32);
            pango_layout_set_font_description(ly, (f.0).0);
            Ok(TextLayout(Rc::new(PangoLayoutAuto(ly))))
        }
    }

    fn clear(&mut self, col: Color) {
        unsafe {
            self.set_color(col);
            cairo_set_operator(self.cx, enums::Operator::Source);
            cairo_paint(self.cx);
        }
    }

    fn set_color(&mut self, col: Color) {
        unsafe { cairo_set_source_rgba(self.cx, col.r as f64, col.g as f64, col.b as f64, col.a as f64); }
    }

    fn stroke_rect(&mut self, rect: Rect, stroke_width: f32) {
        unsafe {
            cairo_set_line_width(self.cx, stroke_width as f64);
            cairo_rectangle(self.cx, rect.x as f64, rect.y as f64, 
                            rect.w as f64, rect.h as f64);
            cairo_stroke(self.cx);
        }
    }

    fn fill_rect(&mut self, rect: Rect) {
        unsafe {
            cairo_rectangle(self.cx, rect.x as f64, rect.y as f64, 
                            rect.w as f64, rect.h as f64);
            cairo_fill(self.cx);
        }
    }

    fn draw_line(&mut self, a: Point, b: Point, stroke_width: f32) {
        unsafe {
            cairo_set_line_width(self.cx, stroke_width as f64);
            cairo_move_to(self.cx, a.x as f64, a.y as f64);
            cairo_line_to(self.cx, b.x as f64, b.y as f64);
            cairo_stroke(self.cx);
        }
    }

    fn draw_text(&mut self, rect: Rect, s: &str, f: &Font) {
        unsafe {
            let ly = pango_layout_new(self.pg);
            pango_layout_set_text(ly, s.as_ptr() as *const i8, s.len() as i32);
            pango_layout_set_font_description(ly, (f.0).0);
            cairo_save(self.cx);
            cairo_move_to(self.cx, rect.x as f64, rect.y as f64);
            pango_cairo_show_layout(self.cx, ly);
            cairo_restore(self.cx);
            g_object_unref(transmute(ly));
        }
    }

    fn draw_text_layout(&mut self, p: Point, txl: &TextLayout) {
        unsafe {
            pango_cairo_update_layout(self.cx, (txl.0).0);
            cairo_save(self.cx);
            cairo_move_to(self.cx, p.x as f64, p.y as f64);
            pango_cairo_show_layout(self.cx, (txl.0).0);
            cairo_restore(self.cx);
        }
    }

    fn translate(&mut self, p: Point) {
        unsafe {
            cairo_translate(self.cx, p.x as f64, p.y as f64);
        }
    }

    fn new(win: &mut winit::Window) -> Result<Self, Box<Error>> {
        unsafe {
            let mut surf = S::new(win)?;
            let cx = cairo_create(surf.surface());
            let pg = pango_cairo_create_context(cx);
            Ok(CairoRenderContext {
                surface: surf, cx, pg
            })
        }
    }

    fn bounds(&self) -> Rect { self.surface.bounds() }

    fn start_paint(&mut self) {
        unsafe {
            cairo_identity_matrix(self.cx);
        }
        self.surface.start_paint()
    }
    fn end_paint(&mut self) { self.surface.end_paint() }

    fn resize(&mut self, w: u32, h: u32) {
        self.surface.resize(w,h);
        unsafe { 
            cairo_destroy(self.cx);
            self.cx = cairo_create(self.surface.surface());
            g_object_unref(transmute(self.pg));
            self.pg = pango_cairo_create_context(self.cx);
        }
    }
}
