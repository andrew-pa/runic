use super::*;
use std::mem::MaybeUninit;
use std::ptr::null_mut;

mod vgu; //handle lowest level COM stuff

pub fn init() {
    unsafe {
        vgu::SetProcessDpiAwareness(2);
    }
}

pub type Font = vgu::Com<vgu::IDWriteTextFormat>;
pub type TextLayout = vgu::Com<vgu::IDWriteTextLayout>;

pub struct RenderContext {
    dwfac: vgu::TextFactory,
    rt: vgu::WindowRenderTarget,
    scb: vgu::Brush,
    dpi: (f32, f32)
}

impl TextLayoutExt for TextLayout {
    fn bounds(&self) -> Rect {
        unsafe {
            let mut metrics: MaybeUninit<vgu::DWRITE_TEXT_METRICS> = MaybeUninit::uninit();
            (*self.p).GetMetrics(metrics.as_mut_ptr());
            let metrics = metrics.assume_init();
            Rect::xywh(metrics.left, metrics.top, metrics.width, metrics.height)
        }
    }

    fn char_bounds(&self, index: usize) -> Rect {
        unsafe {
            let mut ht: MaybeUninit<vgu::DWRITE_HIT_TEST_METRICS> = MaybeUninit::uninit();
            let (mut x, mut y) = (0.0, 0.0);
            (*self.p).HitTestTextPosition(index as u32, 0, &mut x, &mut y, ht.as_mut_ptr());
            let ht = ht.assume_init();
            Rect::xywh(x, y, ht.width, ht.height)
        }
    }

    fn hit_test(&self, p: Point) -> Option<(usize, Rect)> {
        unsafe {
            let mut ht: MaybeUninit<vgu::DWRITE_HIT_TEST_METRICS> = MaybeUninit::uninit();
            let mut inside:i32 = 0;
            let mut trailing:i32 = 0;
            (*self.p).HitTestPoint(p.x, p.y, &mut trailing, &mut inside, ht.as_mut_ptr());
            let ht = ht.assume_init();
            if inside > 0 {
                Some((ht.textPosition as usize, Rect::xywh(ht.left, ht.top, ht.width, ht.height)))
            } else {
                None
            }
        }
    }

    fn color_range(&self, rx: &RenderContext, range: Range<u32>, col: Color) {
        unsafe {
            let r = vgu::DWRITE_TEXT_RANGE { startPosition: range.start, length: range.len() as u32 };
            let brush = vgu::Brush::solid_color(&rx.rt, vgu::D2D1_COLOR_F{r:col.r, g:col.g, b:col.b, a:col.a}).expect("create color brush");
            (*self.p).SetDrawingEffect(::std::mem::transmute(brush.p), r);
        }
    }

    fn style_range(&self, range: Range<u32>, style: FontStyle) {
        unsafe {
            let r = vgu::DWRITE_TEXT_RANGE { startPosition: range.start, length: range.len() as u32 };
            (*self.p).SetFontStyle(match style {
                                     FontStyle::Normal => vgu::DWRITE_FONT_STYLE_NORMAL,
                                     FontStyle::Italic => vgu::DWRITE_FONT_STYLE_ITALIC
                                 }, r);
        }
    }

    fn weight_range(&self, range: Range<u32>, weight: FontWeight) {
        unsafe {
            let r = vgu::DWRITE_TEXT_RANGE { startPosition: range.start, length: range.len() as u32 };
            (*self.p).SetFontWeight(match weight {
                                     FontWeight::Light => vgu::DWRITE_FONT_WEIGHT_LIGHT,
                                     FontWeight::Regular => vgu::DWRITE_FONT_WEIGHT_REGULAR,
                                     FontWeight::Bold => vgu::DWRITE_FONT_WEIGHT_BOLD
                                 }, r);
        }
    }
    
    fn underline_range(&self, range: Range<u32>, ul: bool) {
        unsafe {
            let r = vgu::DWRITE_TEXT_RANGE { startPosition: range.start, length: range.len() as u32 };
            (*self.p).SetUnderline(if ul { 1 } else { 0 }, r);
        }
    }

    fn size_range(&self, range: Range<u32>, size: f32) {
        unsafe {
            let r = vgu::DWRITE_TEXT_RANGE { startPosition: range.start, length: range.len() as u32 };
            (*self.p).SetFontSize(size, r);
        }
    }
}

use winit::platform::windows::WindowExtWindows;
impl RenderContextExt for RenderContext {
    fn new(win: &mut winit::window::Window) -> Result<RenderContext, Box<dyn Error>> {
        let d2fac = vgu::Factory::new()?;
        let dwfac = vgu::TextFactory::new()?;
        let mut dpi: (f32, f32) = (0.0, 0.0);
        //TODO: Winit now has all the APIs needed to do this without using Winapi
        unsafe { (*d2fac.p).GetDesktopDpi(&mut dpi.0, &mut dpi.1); }
        let winit::dpi::PhysicalSize{ width, height } = win.inner_size();
        unsafe {
            let wnd = win.hwnd() as vgu::HWND;
            let mut rect = vgu::RECT {
                left: 0, top: 0,
                right: ((width as f32) * (dpi.0 / 96.0)).ceil() as i32,
                bottom: ((height as f32) * (dpi.1 / 96.0)).ceil() as i32,
            };
            vgu::AdjustWindowRect(&mut rect, vgu::GetWindowLongW(wnd, vgu::GWL_STYLE) as u32, 0);
            vgu::SetWindowPos(wnd, null_mut(), 0, 0,
                rect.right-rect.left, rect.bottom-rect.top,
                vgu::SWP_NOMOVE|vgu::SWP_ASYNCWINDOWPOS);
        }
        let rt = vgu::WindowRenderTarget::new(d2fac.clone(), &win)?;
        unsafe {
            (*rt.p).SetTextAntialiasMode(vgu::D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE);
        }
        let scb = vgu::Brush::solid_color(&rt, vgu::D2D1_COLOR_F{r:0.0,g:0.0,b:0.0,a:1.0})?;
        Ok(RenderContext { dwfac, rt, scb, dpi })
    }

    fn new_font(&self, name: &str, size: f32, weight: FontWeight, style: FontStyle) -> Result<Font, Box<dyn Error>> {
        use windows::vgu::*;
        unsafe {
            let mut txf: MaybeUninit<*mut vgu::IDWriteTextFormat> = MaybeUninit::uninit();
            let mut font_name = name.encode_utf16().collect::<Vec<u16>>();
            font_name.push(0u16);
            font_name.push(0u16);
            (*self.dwfac.p).CreateTextFormat(font_name.as_ptr(), null_mut(), 
                                 match weight {
                                     FontWeight::Light => vgu::DWRITE_FONT_WEIGHT_LIGHT,
                                     FontWeight::Regular => vgu::DWRITE_FONT_WEIGHT_REGULAR,
                                     FontWeight::Bold => vgu::DWRITE_FONT_WEIGHT_BOLD
                                 },
                                 match style {
                                     FontStyle::Normal => vgu::DWRITE_FONT_STYLE_NORMAL,
                                     FontStyle::Italic => vgu::DWRITE_FONT_STYLE_ITALIC
                                 }, vgu::DWRITE_FONT_STRETCH_NORMAL, size, [101u16, 110u16, 45u16, 117u16, 115u16, 0u16, 0u16].as_ptr() /*'en-us'*/, txf.as_mut_ptr())
            .into_result(|| vgu::Com::from_ptr(txf.assume_init())).map_err(Into::into)
        }
    }

    fn new_text_layout(&self, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<dyn Error>> {
        use windows::vgu::*;
        use std::mem::transmute;
        unsafe {
            let mut lo: MaybeUninit<*mut IDWriteTextLayout> = MaybeUninit::uninit();
            let mut txd = text.encode_utf16().collect::<Vec<u16>>();
            txd.push(0u16);
            txd.push(0u16);
            (*self.dwfac.p).CreateTextLayout(txd.as_ptr(), txd.len() as UINT32, f.p, width, height, lo.as_mut_ptr())
                .into_result(|| Com::from_ptr(transmute(lo.assume_init()))).map_err(Into::into)
        }
    }


    fn clear(&mut self, col: Color) {
        unsafe {
            self.rt.Clear(&vgu::D2D1_COLOR_F{r: col.r, g: col.g, b: col.b, a: col.a});
        }
    }

    fn set_color(&mut self, col: Color) {
        unsafe {
            self.scb.set_color(vgu::D2D1_COLOR_F{r:col.r, g:col.g, b:col.b, a:col.a});
        }
    }

    fn stroke_rect(&mut self, rect: Rect, stroke_width: f32) {
        unsafe {
            self.rt.DrawRectangle(&vgu::D2D1_RECT_F{left: rect.x, top: rect.y, right: rect.x+rect.w, bottom: rect.y+rect.h},
                                  self.scb.p, stroke_width, std::ptr::null_mut());
        }
    }
    fn fill_rect(&mut self, rect: Rect) {
        unsafe {
            self.rt.FillRectangle(&vgu::D2D1_RECT_F{left: rect.x, top: rect.y, right: rect.x+rect.w, bottom: rect.y+rect.h},
                                  self.scb.p);
        }
    }
    fn draw_line(&mut self, a: Point, b: Point, stroke_width: f32) {
        unsafe {
            self.rt.DrawLine(vgu::D2D1_POINT_2F{x:a.x, y:a.y}, vgu::D2D1_POINT_2F{x:b.x, y:b.y},
                             self.scb.p, stroke_width, std::ptr::null_mut());
        }
    }
    fn draw_text(&mut self, rect: Rect, s: &str, f: &Font) {
        unsafe {
            let s16 = s.encode_utf16().collect::<Vec<u16>>();
            self.rt.DrawText(s16.as_ptr(), s16.len() as u32,
                f.p, &vgu::D2D1_RECT_F{left: rect.x, top: rect.y, right: rect.x+rect.w, bottom: rect.y+rect.h}, self.scb.p,
                vgu::D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT, vgu::DWRITE_MEASURING_MODE_NATURAL);
        }
    }
    fn draw_text_layout(&mut self, p: Point, txl: &TextLayout) {
        unsafe {
            self.rt.DrawTextLayout(vgu::D2D1_POINT_2F{x:p.x, y:p.y}, txl.p, self.scb.p,
                                   vgu::D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT);
        }
    }

    fn bounds(&self) -> Rect {
        unsafe {
            let s: vgu::D2D1_SIZE_F = (*self.rt.p).GetSize();
            Rect::xywh(0.0, 0.0, s.width, s.height)
        }
    }

    fn translate(&mut self, p: Point) {
        unsafe {
            let s: vgu::D2D1_MATRIX_3X2_F = vgu::D2D1_MATRIX_3X2_F {
                matrix: [ [1.0, 0.0],
                          [0.0, 1.0],
                          [p.x, p.y] ]
            };
            self.rt.SetTransform(&s);
        }
    }

    fn pixels_to_points(&self, p: Point) -> Point {
        Point::xy(p.x * (96.0 / self.dpi.0), p.y * (96.0 / self.dpi.1))
    }

    fn start_paint(&mut self) {
        unsafe {
            self.rt.BeginDraw();
        }
    }

    fn end_paint(&mut self) {
        unsafe {
            self.rt.EndDraw(null_mut(), null_mut());
        }
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.rt.resize(w, h);
    }
}
