use super::{Rect, Point, Color, FontWeight, FontStyle};
use std::error::Error;
use std::os::raw::c_void;
use std::mem::transmute;
use std::rc::Rc;

use objc::runtime::{Class, Object};

use cairo_sys::*;
use pango_sys::*;
use pangocairo_sys::*;
use gobject_sys::g_object_unref;

/*
 * eventually split out the Cairo/Pango part:
 * trait CairoSurface {
 *  fn new(win: &mut winit::Window) -> Self;
 *  fn begin_paint/end_paint();
 *  fn resize(w, h);
 *  fn surface() -> *mut cairo_surface_t;
 * }
 * struct CairoRenderContext<S: CairoSurfaceProvider> {
 * }
 * impl CairoSurface for OSXSurface; etc.
 */

use winit;

use winit::os::macos::WindowExt;

use cairo_context;

pub type Font = cairo_context::Font;
pub type TextLayout = cairo_context::TextLayout;
pub type RenderContext = cairo_context::CairoRenderContext<QuartzCairoSurface>;

pub fn init() { }

pub struct QuartzCairoSurface {
    qgx: *mut Object,
    surf: *mut cairo_surface_t,
    size: (u32, u32), dpi_factor: f32
}

extern "C" {
    fn cairo_surface_get_device_offset(surface: *mut cairo_surface_t, x: *mut f64, y: *mut f64);
    fn cairo_surface_get_device_scale(surface: *mut cairo_surface_t, x: *mut f64, y: *mut f64);
    fn cairo_surface_set_device_offset(surface: *mut cairo_surface_t, x: f64, y: f64);
    fn cairo_surface_set_device_scale(surface: *mut cairo_surface_t, x: f64, y: f64);
}

impl cairo_context::CairoSurface for QuartzCairoSurface {
    fn new(win: &mut winit::Window) -> Result<Self, Box<Error>> where Self: Sized {
        let (width,height) = win.get_inner_size_points().ok_or("")?;
        unsafe {
            let NSGraphicsContext = Class::get("NSGraphicsContext").ok_or("")?;
            let nsgx: *mut Object = msg_send![NSGraphicsContext, graphicsContextWithWindow: win.get_nswindow()];
            let cg: *mut c_void = msg_send![nsgx, graphicsPort];
            let surf = cairo_quartz_surface_create_for_cg_context(
                transmute(cg), width, height);
            let mut offset: (f64, f64) = (0.0, 0.0);
            let mut scale: (f64, f64) = (0.0, 0.0);
            cairo_surface_get_device_offset(surf, &mut offset.0, &mut offset.1);
            cairo_surface_get_device_scale(surf, &mut scale.0, &mut scale.1);
            cairo_surface_set_device_offset(surf, offset.0, offset.1 + height as f64);
            cairo_surface_set_device_scale(surf, scale.0, -scale.1);
            Ok(QuartzCairoSurface{
                qgx: nsgx, surf, size: (width, height), dpi_factor: win.hidpi_factor()
            })
        }
    }

    fn surface(&self) -> *mut cairo_surface_t { self.surf }

    fn bounds(&self) -> Rect { Rect::xywh(0.0,0.0,self.size.0 as f32,self.size.1 as f32) }

    fn start_paint(&mut self) {
    }
    fn end_paint(&mut self) {
        unsafe {
            msg_send![self.qgx, flushGraphics]
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.size = ((width as f32/self.dpi_factor) as u32, (height as f32/self.dpi_factor) as u32);
        unsafe {
            let cg: *mut c_void = msg_send![self.qgx, graphicsPort];
            cairo_surface_destroy(self.surf);
            self.surf = cairo_quartz_surface_create_for_cg_context(
                transmute(cg), width, height);
            let mut offset: (f64, f64) = (0.0, 0.0);
            let mut scale: (f64, f64) = (0.0, 0.0);
            cairo_surface_get_device_offset(self.surf, &mut offset.0, &mut offset.1);
            cairo_surface_get_device_scale(self.surf, &mut scale.0, &mut scale.1);
            cairo_surface_set_device_offset(self.surf, offset.0, offset.1 + height as f64 / self.dpi_factor as f64);
            cairo_surface_set_device_scale(self.surf, scale.0, -scale.1);
        }
    }
}
