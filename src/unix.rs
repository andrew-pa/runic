
use std::error::Error;
use cairo_sys::*;
use winit;
use super::{Rect};

use x11_dl::xlib::*;
use x11_dl::xrender::*;

use winit::os::unix::WindowExt;
use std::mem::transmute;

use cairo_context;

extern "C" {
    fn cairo_xlib_surface_create_with_xrender_format(
        display: *mut Display, drawable: Drawable, screen: *mut Screen, fmt: *mut XRenderPictFormat, w: i32, h: i32) -> *mut cairo_surface_t;
    fn cairo_xlib_surface_create(display: *mut Display, drawable: Drawable,
                                 visual: *mut Visual, w: i32, h: i32) -> *mut cairo_surface_t;
    fn cairo_xlib_surface_set_size(surface: *mut cairo_surface_t, w: i32, h: i32);
}

pub struct UnixCairoSurface {
    surface: *mut cairo_surface_t,
    size: (u32, u32)
}

impl cairo_context::CairoSurface for UnixCairoSurface {
    fn new(win: &mut winit::Window) -> Result<Self, Box<Error>> where Self: Sized {
        unsafe {
            let x = Xlib::open()?;
            let xrndr = Xrender::open()?;
            let (w,h) = win.get_inner_size_points().ok_or("")?;
            let _display = win.get_xlib_display();
            let _xwin = win.get_xlib_window();
            let _sid = win.get_xlib_screen_id();
            let display = transmute(win.get_xlib_display().unwrap());
            let drawable: XID = transmute(win.get_xlib_window().unwrap());
            let screen_id: usize = transmute(win.get_xlib_screen_id().unwrap());
            let screen = (x.XScreenOfDisplay)(display, screen_id as i32);
            let fmt = (xrndr.XRenderFindStandardFormat)(display, 0);
            /*let surf = cairo_xlib_surface_create_with_xrender_format(
                display, drawable, screen, fmt, w as i32, h as i32);*/
            let surf = cairo_xlib_surface_create(display, drawable, 
                            (x.XDefaultVisual)(display, screen_id as i32), w as i32, h as i32);
            println!("surf = {:?}", surf);
            Ok(UnixCairoSurface { surface: surf, size: (w,h) })
        }
    }
    fn start_paint(&mut self) {
    }
    fn end_paint(&mut self) {
        unsafe {
            cairo_surface_flush(self.surface);
        }
    }
    fn resize(&mut self, w: u32, h: u32) {
        self.size = (w,h);
        unsafe {
            cairo_xlib_surface_set_size(self.surface, w as i32, h as i32);
        }
    }
    fn surface(&self) -> *mut cairo_surface_t { self.surface }
    fn bounds(&self) -> Rect { Rect::xywh(0.0, 0.0, self.size.0 as f32, self.size.1 as f32) }
}

pub type Font = cairo_context::Font;
pub type TextLayout = cairo_context::TextLayout;
pub type RenderContext = cairo_context::CairoRenderContext<UnixCairoSurface>;

pub fn init() { }
