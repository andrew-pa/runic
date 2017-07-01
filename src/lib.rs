#![feature(i128_type)]

use std::error::Error;

#[cfg(target_os = "windows")]
mod vgu;

pub struct Rect {
    x: f32, y: f32, w: f32, h: f32
}

impl Rect {
    pub fn xywh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }
}

pub struct Color {
    r: f32, g: f32, b: f32, a: f32
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }
}

#[derive(Debug)]
pub enum Event {
    Resize(u32,u32),
    Key
}

#[cfg(target_os = "windows")]
pub struct RenderContext {
    d2fac: vgu::Factory,
    rt: vgu::WindowRenderTarget,
}

impl RenderContext {
    pub fn clear(&mut self, col: Color) {
        #[cfg(target_os = "windows")]
        unsafe {
            self.rt.Clear(&vgu::D2D1_COLOR_F{r: col.r, g: col.g, b: col.b, a: col.a});
        }
    }
    pub fn stroke_rect(&self, rect: Rect, col: Color, stroke_width: f32) { }
}

pub trait App {
    fn paint(&self, rx: &mut RenderContext);
    fn event(&mut self, e: Event);
}

#[cfg(target_os = "windows")]
type NativeWindow = vgu::Window;

pub struct Window<'app> {
    app: &'app mut App,
    rx: RenderContext,
    nwin: NativeWindow
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn global_winproc(win: vgu::HWND, msg: vgu::UINT, w: vgu::WPARAM, l: vgu::LPARAM) -> vgu::LRESULT {
    use std::ptr::{null_mut,null};
    let pw = vgu::GetWindowLongPtrW(win, 0);
    if pw == 0 { return vgu::DefWindowProcW(win, msg, w, l); }
    let rwin: &mut Window = std::mem::transmute(pw);
    assert_eq!(win, rwin.nwin.hndl);
    match msg {
        vgu::WM_CREATE => { vgu::SetWindowLongPtrW(win, 0, 0); 0 }
        vgu::WM_PAINT => {
            rwin.rx.rt.BeginDraw();
            rwin.app.paint(&mut rwin.rx);
            rwin.rx.rt.EndDraw(null_mut(), null_mut());
            1
        }
        vgu::WM_SIZE => { rwin.app.event(Event::Resize(vgu::GET_X_LPARAM(l) as u32, vgu::GET_Y_LPARAM(l) as u32)); 0 }
        vgu::WM_DESTROY => { vgu::PostQuitMessage(0); 1 }
        _ => { vgu::DefWindowProcW(win, msg, w, l) }
    }
}

impl<'app> Window<'app> {
    pub fn new(title: &str, width: usize, height: usize, app: &'app mut App) -> Result<Self, Box<Error>> {
        let nwin = { 
            #[cfg(target_os = "windows")]
            vgu::Window::new((width as i32, height as i32), Some(global_winproc))?
        }; 
        let rx = {
            #[cfg(target_os = "windows")]
            {
                let d2fac = vgu::Factory::new()?;
                let rt = vgu::WindowRenderTarget::new(d2fac.clone(), &nwin)?;
                RenderContext { d2fac, rt }
            }
        };
        let mut win = Window {
            app, rx,
            nwin 
        };
        Ok(win)
    }

    pub fn show(&mut self)  {
        #[cfg(target_os = "windows")]
        unsafe {
            vgu::SetWindowLongPtrW(self.nwin.hndl, 0, std::mem::transmute(self));
            vgu::Window::message_loop();
        }
    }
}
