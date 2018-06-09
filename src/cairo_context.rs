use *;

use std::error::Error;
use std::os::raw::c_void;
use std::mem::transmute;
use std::rc::Rc;

use cairo_sys::*;
use pango_sys::*;
use pangocairo_sys::*;
use gobject_sys::{g_object_unref, g_object_ref};

struct PangoFontDesc(*mut PangoFontDescription);

impl Drop for PangoFontDesc {
    fn drop(&mut self) {
        unsafe {
            pango_font_description_free(self.0);
        }
    }
}

#[derive(Clone)]
pub struct Font(Rc<PangoFontDesc>);

struct GObject<T>(*mut T);

impl<T> Clone for GObject<T>{
    fn clone(&self) -> GObject<T> {
        unsafe { g_object_ref(transmute(self.0)); }
        GObject(self.0)
    }
}

impl<T> Drop for GObject<T>{
    fn drop(&mut self) {
        unsafe {
            g_object_unref(transmute(self.0));
        }
    }
}

fn convert_weight(weight: FontWeight) -> pango_sys::PangoWeight {
    match weight {
        FontWeight::Light => PANGO_WEIGHT_LIGHT,
        FontWeight::Regular => PANGO_WEIGHT_MEDIUM,
        FontWeight::Bold => PANGO_WEIGHT_BOLD
    }
}

fn convert_style(style: FontStyle) -> pango_sys::PangoStyle {
    match style {
        FontStyle::Normal => PANGO_STYLE_NORMAL,
        FontStyle::Italic => PANGO_STYLE_ITALIC
    }
}

pub struct TextLayout(Rc<GObject<PangoLayout>>);

impl Clone for TextLayout {
    fn clone(&self) -> Self {
        TextLayout(self.0.clone())
    }
}
    
impl TextLayoutExt for TextLayout {
    fn bounds(&self) -> Rect {
        let mut w = 0i32;
        let mut h = 0i32;
        unsafe {
            pango_layout_get_pixel_size((self.0).0, &mut w, &mut h);
        }
        Rect::xywh(0.0, 0.0, w as f32, h as f32)
    }
    fn char_bounds(&self, index: usize) -> Rect {
            let mut rect: PangoRectangle = PangoRectangle{x:0,y:0,width:0,height:0};
        unsafe {
            pango_layout_index_to_pos((self.0).0, index as i32, &mut rect);
        }
        let ps = 1.0 / PANGO_SCALE as f32;
        Rect::xywh(rect.x as f32 * ps, rect.y as f32 * ps, rect.width as f32 * ps, rect.height as f32 * ps)
    }
    
    fn hit_test(&self, p: Point) -> Option<(usize, Rect)> {
        let mut index: i32 = 0;
        let mut trailing : i32 = 0;
        unsafe {
            if pango_layout_xy_to_index((self.0).0, (p.x * PANGO_SCALE as f32) as i32, (p.y * PANGO_SCALE as f32) as i32, &mut index, &mut trailing) > 0 {
                Some((index as usize, self.char_bounds(index as usize)))
            } else {
                None
            }
        }
    }

    fn color_range(&self, _: &RenderContext, range: Range<u32>, col: Color) {
        unsafe {
            let mut attrs = pango_layout_get_attributes((self.0).0);
            if attrs == std::ptr::null_mut() {
                attrs = pango_attr_list_new();
                pango_layout_set_attributes((self.0).0, attrs);
            }
            let mut attr = pango_attr_foreground_new((col.r*65535.0) as u16, (col.g*65535.0) as u16, (col.b*65535.0) as u16);
            (*attr).start_index = range.start;
            (*attr).end_index = range.end;
            pango_attr_list_change(attrs, attr);
        }
    }
    fn style_range(&self, range: Range<u32>, style: FontStyle) {
        unsafe {
            let mut attrs = pango_layout_get_attributes((self.0).0);
            if attrs == std::ptr::null_mut() {
                attrs = pango_attr_list_new();
                pango_layout_set_attributes((self.0).0, attrs);
            }
            let mut attr = pango_attr_style_new(convert_style(style));
            (*attr).start_index = range.start;
            (*attr).end_index = range.end;
            pango_attr_list_change(attrs, attr);
        }
    }
    fn weight_range(&self, range: Range<u32>, weight: FontWeight) {
        unsafe {
            let mut attrs = pango_layout_get_attributes((self.0).0);
            if attrs == std::ptr::null_mut() {
                attrs = pango_attr_list_new();
                pango_layout_set_attributes((self.0).0, attrs);
            }
            let mut attr = pango_attr_weight_new(convert_weight(weight));
            (*attr).start_index = range.start;
            (*attr).end_index = range.end;
            pango_attr_list_change(attrs, attr);
        }
    }
    fn underline_range(&self, range: Range<u32>, ul: bool) {
        unsafe {
            let mut attrs = pango_layout_get_attributes((self.0).0);
            if attrs == std::ptr::null_mut() {
                attrs = pango_attr_list_new();
                pango_layout_set_attributes((self.0).0, attrs);
            }
            let mut attr = pango_attr_underline_new(if ul {
                PANGO_UNDERLINE_SINGLE
            } else {
                PANGO_UNDERLINE_NONE
            });
            (*attr).start_index = range.start;
            (*attr).end_index = range.end;
            pango_attr_list_change(attrs, attr);
        }
    }
    fn size_range(&self, range: Range<u32>, size: f32) {
        unsafe {
            let mut attrs = pango_layout_get_attributes((self.0).0);
            if attrs == std::ptr::null_mut() {
                attrs = pango_attr_list_new();
                pango_layout_set_attributes((self.0).0, attrs);
            }
            let mut attr = pango_attr_size_new((size * PANGO_SCALE as f32) as i32);
            (*attr).start_index = range.start;
            (*attr).end_index = range.end;
            pango_attr_list_change(attrs, attr);
        }
    }
}


pub trait CairoSurface {
    fn new(win: &mut winit::Window) -> Result<Self, Box<Error>> where Self: Sized;
    fn start_paint(&mut self);
    fn end_paint(&mut self);
    fn resize(&mut self, w: u32, h: u32);
    fn surface(&self) -> *mut cairo_surface_t;
    fn bounds(&self) -> Rect;
    fn pixels_to_points(&self, p: Point) -> Point;
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

            let rsname = name.as_bytes();
            let mut szname = Vec::new();
            for i in 0..rsname.len() {
                szname.push(rsname[i]);
            }
            szname.push(0);
            
            pango_font_description_set_family(fd, szname.as_ptr() as *const i8);
            pango_font_description_set_size(fd, (size * PANGO_SCALE as f32) as i32);
            pango_font_description_set_weight(fd, convert_weight(weight));
            pango_font_description_set_style(fd, convert_style(style));
            
            Ok(Font(Rc::new(PangoFontDesc(fd))))
        }
    }

    fn new_text_layout(&self, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<Error>> {
        unsafe {
            let ly = pango_layout_new(self.pg);
            pango_layout_set_text(ly, text.as_ptr() as *const i8, text.len() as i32);
            pango_layout_set_font_description(ly, (f.0).0);
            Ok(TextLayout(Rc::new(GObject(ly))))
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

    fn pixels_to_points(&self, p: Point) -> Point { self.surface.pixels_to_points(p) }
}
