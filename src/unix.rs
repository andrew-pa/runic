
use std::error::Error;
use cairo_sys::*;
use winit;
use super::Rect;

#[cfg(feature = "x11")]
use x11_dl::xlib::*;
#[cfg(feature = "x11")]
use x11_dl::xrender::*;

#[cfg(feature = "wayland")]
use servo_egl::egl;


use winit::platform::unix::WindowExt;
use std::mem::transmute;

use cairo_context;

#[cfg(feature = "x11")]
extern "C" {
    fn cairo_xlib_surface_create_with_xrender_format(
        display: *mut Display, drawable: Drawable, screen: *mut Screen, fmt: *mut XRenderPictFormat, w: i32, h: i32) -> *mut cairo_surface_t;
    fn cairo_xlib_surface_create(display: *mut Display, drawable: Drawable,
                                 visual: *mut Visual, w: i32, h: i32) -> *mut cairo_surface_t;
    fn cairo_xlib_surface_set_size(surface: *mut cairo_surface_t, w: i32, h: i32);
}

#[cfg(feature = "wayland")]
extern "C" {
    fn cairo_gl_surface_set_size(surface: *mut cairo_surface_t, w: i32, h: i32);
    fn cairo_gl_surface_swapbuffers(surface: *mut cairo_surface_t) -> i32;

    fn cairo_egl_device_create(display: egl::EGLDisplay, context: egl::EGLContext) -> *mut cairo_device_t;
    fn cairo_gl_surface_create_for_egl(device: *mut cairo_device_t, egl_surface: *mut _, w: i32, h: i32) -> *mut cairo_surface_t;
}

pub struct UnixCairoSurface {
    surface: *mut cairo_surface_t,
    wayland_objects: Option<(egl::EGLDisplay, egl::EGLSurface, wayland_egl::WlEglSurface)>,
    size: (u32, u32)
}

impl cairo_context::CairoSurface for UnixCairoSurface {
    fn new(win: &mut winit::Window) -> Result<Self, Box<Error>> where Self: Sized {
        if cfg!(feature = "wayland") && let Some(wayland_display) = win.wayland_display() {
            #[cfg(feature = "wayland")]
            unsafe {
                // initialize EGL
                let display = egl::GetDisplay(wayland_display);
                let mut major: MaybeUninit<egl::EGLInt> = MaybeUninit::uninit();
                let mut minor: MaybeUninit<egl::EGLInt> = MaybeUninit::uninit();
                if !egl::Initialize(display, major.as_mut_ptr(), minor.as_mut_ptr()) {
                    return Err("failed to initialize EGL".into());
                }
                println!("EGL major.minor = {}.{}", major.assume_init(), minor.assume_init());

                if !egl::BindAPI(egl::EGL_OPENGL_API) {
                    return Err("failed to bind OpenGL API".into());
                }

                let config_attrbs = [
                    egl::EGL_SURFACE_TYPE, egl::EGL_WINDOW_BIT,
                    egl::EGL_RED_SIZE, 1,
                    egl::EGL_GREEN_SIZE, 1,
                    egl::EGL_BLUE_SIZE, 1,
                    egl::EGL_ALPHA_SIZE, 1,
                    egl::EGL_DEPTH_SIZE, 1,
                    egl::EGL_RENDERABLE_TYPE, egl::EGL_OPENGL_BIT,
                    egl::EGL_NONE
                ];

                let mut config: MaybeUninit<egl::EGLConfig> = MaybeUninit::uninit();
                let mut config_num: MaybeUninit<egl::EGLint> = MaybeUninit::uninit();
                if !egl::ChooseConfig(display, attribs.as_ptr(), config.as_mut_ptr(), 1, config_num.as_mut_ptr())
                    || config_num.assume_init() != 1 {
                    return Err("failed to get a sutiable EGL configuration".into());
                }
                let config = config.assume_init();

                let context = match egl::CreateContext(display, config, egl::EGL_NO_CONTEXT, null()) {
                    null() => { return Err("failed to create EGL context".into()) },
                    p => p,
                };

                let cdevice = cairo_egl_device_create(display, context);
                if cairo_device_status(cdevice) != CAIRO_STATUS_SUCCESS) {
                    return Err("failed to create cairo device".into());
                }

                let PhysicalSize {width, height} = win.inner_size();

                let egl_window = wayland_egl::WlEglSurface::new_from_raw(win.wayland_surface().unwrap(), width, height);

                let egl_surf = egl::CreateWindowSurface(display, config, egl_window.as_ptr(), null());

                Ok(UnixCairoSurface {
                    surface: cairo_gl_surface_create_for_egl(cdevice, egl_surf, width, height),
                    wayland_objects: Some((display, egl_surf, egl_window)),
                    size: (width, height)
                })
            }
        } else if cfg!(feature = "x11") && win.xlib_display().is_some() {
            #[cfg(feature = "x11")]
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
            }
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
            self.wayland_objects.as_ref().unwrap().2.resize(w as i32, h as i32, 0, 0);
            cairo_gl_surface_set_size(self.surface, w as i32, h as i32);
        }
    }
    fn surface(&self) -> *mut cairo_surface_t { self.surface }
    fn bounds(&self) -> Rect { Rect::xywh(0.0, 0.0, self.size.0 as f32, self.size.1 as f32) }
}

pub type Font = cairo_context::Font;
pub type TextLayout = cairo_context::TextLayout;
pub type RenderContext = cairo_context::CairoRenderContext<UnixCairoSurface>;

pub fn init() { }
