use super::*;

mod vgu;

pub struct RenderContext {
    d2fac: vgu::Factory,
    rt: vgu::WindowRenderTarget,
    scb: vgu::Brush
}

impl RenderContext {
    fn new(d2fac: vgu::Factory, nwin: &vgu::Window) -> Result<RenderContext, Box<Error>> {
        let rt = vgu::WindowRenderTarget::new(d2fac.clone(), &nwin)?;
        let scb = vgu::Brush::solid_color(rt.clone(), vgu::D2D1_COLOR_F{r:0.0,g:0.0,b:0.0,a:1.0})?;
        Ok(RenderContext { d2fac, rt, scb })
    }

    pub fn clear(&mut self, col: Color) {
        unsafe {
            self.rt.Clear(&vgu::D2D1_COLOR_F{r: col.r, g: col.g, b: col.b, a: col.a});
        }
    }
    pub fn stroke_rect(&mut self, rect: Rect, col: Color, stroke_width: f32) {
        unsafe {
            self.scb.set_color(vgu::D2D1_COLOR_F{r:col.r, g:col.g, b:col.b, a:col.a});
            self.rt.DrawRectangle(&vgu::D2D1_RECT_F{left: rect.x, top: rect.y, right: rect.x+rect.w, bottom: rect.y+rect.h},
                                  self.scb.p, stroke_width, std::ptr::null_mut());
        }
    }
    pub fn fill_rect(&mut self, rect: Rect, col: Color) {
        unsafe {
            self.scb.set_color(vgu::D2D1_COLOR_F{r:col.r, g:col.g, b:col.b, a:col.a});
            self.rt.FillRectangle(&vgu::D2D1_RECT_F{left: rect.x, top: rect.y, right: rect.x+rect.w, bottom: rect.y+rect.h},
                                  self.scb.p);
        }
    }
    pub fn draw_line(&mut self, a: Point, b: Point, col: Color, stroke_width: f32) {
        unsafe {
            self.scb.set_color(vgu::D2D1_COLOR_F{r:col.r, g:col.g, b:col.b, a:col.a});
            self.rt.DrawLine(vgu::D2D1_POINT_2F{x:a.x, y:a.y}, vgu::D2D1_POINT_2F{x:b.x, y:b.y},
                             self.scb.p, stroke_width, std::ptr::null_mut());
        }
    }

}

type NativeWindow = vgu::Window;

pub struct Window<'app> {
    app: &'app mut App,
    rx: RenderContext,
    nwin: NativeWindow
}

impl Point {
    fn from_lparam(l: vgu::LPARAM) -> Point {
        Point { x: vgu::GET_X_LPARAM(l) as f32, y: vgu::GET_Y_LPARAM(l) as f32 }
    }
    pub fn to_dip(&self, rx: &mut RenderContext) -> Point {
        let mut dpi: (f32, f32) = (0.0, 0.0);
        unsafe { rx.rt.GetDpi(&mut dpi.0, &mut dpi.1); }
        Point { x: (self.x * 96.0) / dpi.0, y: (self.y * 96.0) / dpi.1 }
    }
}

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
        vgu::WM_SIZE => {
            let (w, h) = (vgu::GET_X_LPARAM(l) as u32, vgu::GET_Y_LPARAM(l) as u32);
            rwin.rx.rt.resize(w, h);
            rwin.app.event(Event::Resize(w, h));
            0
        },
        vgu::WM_MOUSEMOVE => {
            rwin.app.event(Event::MouseMove(Point::from_lparam(l).to_dip(&mut rwin.rx), 
                                            if w & 0x0001 != 0 { Some(MouseButton::Left) }
                                            else if w & 0x0002 != 0 { Some(MouseButton::Right) }
                                            else if w & 0x0010 != 0 { Some(MouseButton::Middle) }
                                            else { None }));
            0
        },
        vgu::WM_LBUTTONDOWN => { rwin.app.event(Event::MouseDown(Point::from_lparam(l).to_dip(&mut rwin.rx), MouseButton::Left)); 0 },
        vgu::WM_LBUTTONUP => { rwin.app.event(Event::MouseUp(Point::from_lparam(l).to_dip(&mut rwin.rx), MouseButton::Left)); 0 },
        vgu::WM_RBUTTONDOWN => { rwin.app.event(Event::MouseDown(Point::from_lparam(l).to_dip(&mut rwin.rx), MouseButton::Right)); 0 },
        vgu::WM_RBUTTONUP => { rwin.app.event(Event::MouseUp(Point::from_lparam(l).to_dip(&mut rwin.rx), MouseButton::Right)); 0 },
        vgu::WM_MBUTTONDOWN => { rwin.app.event(Event::MouseDown(Point::from_lparam(l).to_dip(&mut rwin.rx), MouseButton::Middle)); 0 },
        vgu::WM_MBUTTONUP => { rwin.app.event(Event::MouseUp(Point::from_lparam(l).to_dip(&mut rwin.rx), MouseButton::Middle)); 0 },
        vgu::WM_DESTROY => { vgu::PostQuitMessage(0); 1 }
        _ => { vgu::DefWindowProcW(win, msg, w, l) }
    }
}

impl<'app> Window<'app> {
    pub fn new(title: &str, width: usize, height: usize, app: &'app mut App) -> Result<Self, Box<Error>> {
        unsafe { vgu::SetProcessDpiAwareness(2); }
        let mut d2fac = vgu::Factory::new()?;
        let mut dpi: (f32, f32) = (0.0, 0.0);
        unsafe { d2fac.GetDesktopDpi(&mut dpi.0, &mut dpi.1); }
        let nwin = vgu::Window::new(title, (
                ((width as f32) * (dpi.0 / 96.0)).ceil() as i32,
                ((height as f32) * (dpi.1 / 96.0)).ceil() as i32), Some(global_winproc))?;
        let mut rx = RenderContext::new(d2fac, &nwin)?;
        let mut win = Window {
            app, rx,
            nwin 
        };
        Ok(win)
    }

    pub fn show(&mut self)  {
        unsafe {
            vgu::SetWindowLongPtrW(self.nwin.hndl, 0, std::mem::transmute(self));
            vgu::Window::message_loop();
        }
    }
}
