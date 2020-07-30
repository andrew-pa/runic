
use std::error::Error;
use cairo_sys::*;
use winit;
use super::{Point, Rect};

#[cfg(feature = "x11")]
use x11_dl::xlib::*;
#[cfg(feature = "x11")]
use x11_dl::xrender::*;

#[cfg(feature = "wayland")]
use egl::egl;


use winit::platform::unix::WindowExtUnix;
use std::mem::{transmute, MaybeUninit};
use std::ptr::{null, null_mut};

use crate::cairo_context;

#[cfg(feature = "x11")]
extern "C" {
    fn cairo_xlib_surface_create_with_xrender_format(
        display: *mut Display, drawable: Drawable, screen: *mut Screen, fmt: *mut XRenderPictFormat, w: i32, h: i32) -> *mut cairo_surface_t;
    fn cairo_xlib_surface_create(display: *mut Display, drawable: Drawable,
                                 visual: *mut Visual, w: i32, h: i32) -> *mut cairo_surface_t;
    fn cairo_xlib_surface_set_size(surface: *mut cairo_surface_t, w: i32, h: i32);
}

#[cfg(feature = "wayland")]
#[repr(C)]
struct cairo_device_t(std::ffi::c_void);

#[cfg(feature = "wayland")]
#[link(name = "EGL")]
#[link(name = "GL")]
#[link(name = "cairo")]
extern "C" {
    fn cairo_device_status(device: *mut cairo_device_t) -> i32;
    fn cairo_gl_surface_set_size(surface: *mut cairo_surface_t, w: i32, h: i32);
    fn cairo_gl_surface_swapbuffers(surface: *mut cairo_surface_t) -> i32;

    fn cairo_egl_device_create(display: egl::EGLDisplay, context: egl::EGLContext) -> *mut cairo_device_t;
    fn cairo_gl_surface_create_for_egl(device: *mut cairo_device_t, egl_surface: egl::EGLSurface, w: i32, h: i32) -> *mut cairo_surface_t;
}


pub struct UnixCairoSurface {
    surface: *mut cairo_surface_t,
    wayland_objects: Option<(egl::EGLDisplay, egl::EGLSurface, *mut wayland_sys::egl::wl_egl_window)>,
    size: (u32, u32)
}

impl cairo_context::CairoSurface for UnixCairoSurface {
    fn new(win: &mut winit::window::Window) -> Result<Self, Box<dyn Error>> where Self: Sized {
        if cfg!(feature = "wayland") && win.wayland_display().is_some() {
            #[cfg(feature = "wayland")]
            unsafe {
                use winit::dpi::PhysicalSize;
                use wayland_sys::egl::*;

                // initialize EGL
                let display = egl::GetDisplay(win.wayland_display().unwrap());
                let mut major: egl::EGLint = 0;
                let mut minor: egl::EGLint = 0;
                let egr = egl::Initialize(display, &mut major, &mut minor); 
                if egr != 1 {
                    println!("egl error = {:x}, returned = {:x}", egl::GetError(), egr);
                    return Err("failed to initialize EGL".into());
                }
                //println!("EGL major.minor = {}.{}", major, minor);

                let egr = egl::BindAPI(0x30a2 /*OpenGL*/);
                if egr != 1 {
                    println!("egl error = {:x}, returned = {:x}", egl::GetError(), egr);
                    return Err("failed to bind OpenGL API".into());
                }

                let config_attribs = [
                    egl::EGL_SURFACE_TYPE, egl::EGL_WINDOW_BIT,
                    egl::EGL_RED_SIZE, 1,
                    egl::EGL_GREEN_SIZE, 1,
                    egl::EGL_BLUE_SIZE, 1,
                    egl::EGL_ALPHA_SIZE, 1,
                    egl::EGL_DEPTH_SIZE, 1,
                    egl::EGL_RENDERABLE_TYPE, /*egl::EGL_OPENGL_BIT*/ 0x0008,
                    egl::EGL_NONE
                ];

                let mut config: MaybeUninit<egl::EGLConfig> = MaybeUninit::uninit();
                let mut config_num = 0;
                let egr = egl::ChooseConfig(display, config_attribs.as_ptr() as *mut i32, &mut *config.as_mut_ptr(), 1, &mut config_num); 
                if egr != 1 || config_num != 1 {
                    println!("egl error = {:x}, returned = {:x}, #c = {}", egl::GetError(), egr, config_num);
                    return Err("failed to get a sutiable EGL configuration".into());
                }
                let config = config.assume_init();

                let context = match egl::CreateContext(display, config, null_mut(), null_mut()) {
                    p if p == null_mut() => { return Err("failed to create EGL context".into()) },
                    p => p,
                };

                let cdevice = cairo_egl_device_create(display, context);
                if cairo_device_status(cdevice) != 0 {
                    return Err("failed to create cairo device".into());
                }

                let PhysicalSize {width, height} = win.inner_size();

                let egl_window = (wayland_sys::egl::WAYLAND_EGL_HANDLE.wl_egl_window_create)(
                                               win.wayland_surface().unwrap() as *mut wayland_sys::client::wl_proxy, width as i32, height as i32);

                let egl_surf = egl::CreateWindowSurface(display, config, egl_window as *mut std::ffi::c_void, null());

                Ok(UnixCairoSurface {
                    surface: cairo_gl_surface_create_for_egl(cdevice, egl_surf, width as i32, height as i32),
                    wayland_objects: Some((display, egl_surf, egl_window)),
                    size: (width, height)
                })
            }
        } else if cfg!(feature = "x11") && win.xlib_display().is_some() {
            /*#[cfg(feature = "x11")]
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
                Ok(UnixCairoSurface { surface: surf, wayland_objects: None, size: (w,h) })
            }*/panic!();
        } else {
            Err("no window system found".into())
        }
    }
    fn start_paint(&mut self) {
    }
    fn end_paint(&mut self) {
        unsafe {
            cairo_surface_flush(self.surface);
            #[cfg(feature = "wayland")]
            cairo_gl_surface_swapbuffers(self.surface);
        }
    }
    fn resize(&mut self, w: u32, h: u32) {
        self.size = (w,h);
        #[cfg(feature = "x11")]
        unsafe {
            cairo_xlib_surface_set_size(self.surface, w as i32, h as i32);
        }
        #[cfg(feature = "wayland")]
        unsafe {
            (wayland_sys::egl::WAYLAND_EGL_HANDLE.wl_egl_window_resize)(
                self.wayland_objects.as_ref().unwrap().2, w as i32, h as i32, 0, 0);
            cairo_gl_surface_set_size(self.surface, w as i32, h as i32);
        }
    }
    fn surface(&self) -> *mut cairo_surface_t { self.surface }
    fn bounds(&self) -> Rect { Rect::xywh(0.0, 0.0, self.size.0 as f32, self.size.1 as f32) }
    fn pixels_to_points(&self, p: Point) -> Point { p }
}

pub type Font = cairo_context::Font;
pub type TextLayout = cairo_context::TextLayout;
pub type RenderContext = cairo_context::CairoRenderContext<UnixCairoSurface>;

pub fn init() { }
