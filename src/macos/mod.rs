
use super::*;
use std::error::Error;
use std::os::raw::c_void;

use objc::runtime::{Class, Object};

extern crate cairo_sys;
use self::cairo_sys::*;

use winit::os::macos::WindowExt;

pub fn init() { }

pub struct Font;
pub struct TextLayout;

impl TextLayoutExt for TextLayout {
    fn bounds(&self) -> Rect { Rect::xywh(0.0,0.0,0.0,0.0) }
    fn char_bounds(&self, index: usize) -> Rect { Rect::xywh(0.0, 0.0,0.0,0.0) }
}

pub struct RenderContext {
    qgx: *mut Object,
    surf: *mut cairo_surface_t,
    cx: *mut cairo_t
}

use std::mem::transmute;

impl RenderContextExt for RenderContext {
    fn new(win: &winit::Window) -> Result<Self, Box<Error>> where Self: Sized {
        let (width,height) = win.get_inner_size_pixels().ok_or("")?;
        unsafe {
            let NSGraphicsContext = Class::get("NSGraphicsContext").ok_or("")?;
            let nsgx: *mut Object = msg_send![NSGraphicsContext, graphicsContextWithWindow: win.get_nswindow()];
            let cg: *mut c_void = msg_send![nsgx, graphicsPort];
            let surf = cairo_quartz_surface_create_for_cg_context(
                transmute(cg), width, height);
            let cx = cairo_create(surf);
            cairo_translate(cx, 0.0, height as f64 / 2.0);
            cairo_scale(cx, 1.0, -1.0);
            Ok(RenderContext{
                qgx: nsgx, surf, cx
            })
        }
    }

    fn new_font(&self, name: &str, size: f32, weight: FontWeight, style: FontStyle) -> Result<Font, Box<Error>> {
        Ok(Font)
    }

    fn new_text_layout(&self, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<Error>> {
        Ok(TextLayout)
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

    fn draw_text(&mut self, rect: Rect, s: &str, f: &Font) {}

    fn draw_text_layout(&mut self, p: Point, txl: &TextLayout) {}

    fn translate(&mut self, p: Point) {}

    fn bounds(&self) -> Rect { Rect::xywh(0.0,0.0,0.0,0.0) }

    fn start_paint(&mut self) {}
    fn end_paint(&mut self) {
        unsafe {
            msg_send![self.qgx, flushGraphics]
        }
    }
    fn resize(&mut self, width: u32, height: u32) {
        unsafe {
            let cg: *mut c_void = msg_send![self.qgx, graphicsPort];
            cairo_surface_destroy(self.surf);
            self.surf = cairo_quartz_surface_create_for_cg_context(
                transmute(cg), width, height);
            cairo_destroy(self.cx);
            self.cx = cairo_create(self.surf);
            cairo_translate(self.cx, 0.0, height as f64 / 2.0);
            cairo_scale(self.cx, 1.0, -1.0);
        }
    }
}
