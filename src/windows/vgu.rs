#![allow(dead_code)]
#![allow(non_upper_case_globals)]
extern crate winapi;
pub use self::winapi::*;
pub use self::winapi::shared::minwindef::*;
pub use self::winapi::shared::basetsd::*;
pub use self::winapi::um::winbase::*;
pub use self::winapi::um::winuser::*;
pub use self::winapi::shared::windef::*;
pub use self::winapi::shared::guiddef::*;
pub use self::winapi::um::unknwnbase::*;
pub use self::winapi::shared::winerror::*;
pub use self::winapi::um::d2d1::*;
pub use self::winapi::um::d2dbasetypes::*;
pub use self::winapi::um::dwrite::*;
pub use self::winapi::um::dcommon::*;
pub use self::winapi::shared::dxgiformat::*;
pub use self::winapi::um::errhandlingapi::*;
pub use self::winapi::ctypes::*;


use std::fmt;
use std::ops;
use std::error::Error;
use std::ptr::{null_mut, null};
use std::mem::{uninitialized, transmute};

#[derive(Debug)]
pub struct HResultError {
    res: HRESULT
}

impl HResultError {
    pub fn new(hr: HRESULT) -> HResultError { HResultError { res: hr } }
    pub fn last_win32_error() -> HResultError {
        unsafe {
            HResultError { res: GetLastError() as i32 }
        }
    }
}

impl Error for HResultError {
    fn description(&self) -> &str { "Windows error" }
}

impl fmt::Display for HResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HRESULT 0x{:x} {:?}", self.res, self.res)
    }
}

pub trait IntoResult<E> {
    fn into_result<T, F: FnOnce() -> T>(self, f: F) -> Result<T, E>; 
}

impl IntoResult<HResultError> for HRESULT {
    fn into_result<T, F: FnOnce() -> T>(self, f: F) -> Result<T, HResultError> {
        match self {
            S_OK => Ok(f()),
            v => Err(HResultError::new(v))
        }
    }
}

/*impl RECT {
    fn zero() -> Rect { Rect { 0,0,0,0 } }
    fn extents(width: i32, height: i32) -> Rect { Rect { 0,width,0,height } }
    fn new(x: i32, y: i32, w: i32, h: i32) -> Rect { Rect { x, x+w, y, y+h } }
}*/
pub struct Com<T> {
    pub punk: *mut IUnknown,
    pub p: *mut T
}

impl<T> Com<T> {
    pub fn from_ptr(p: *mut T) -> Com<T> {
        Com { punk: p as *mut IUnknown, p: p }
    }

    pub fn query_interface<U>(&self, id: IID) -> Result<Com<U>, HResultError> {
        unsafe {
            let mut up: *mut U = uninitialized();
            (*self.punk).QueryInterface(&id, (&mut up as *mut *mut U) as *mut *mut c_void).into_result(|| Com { punk: self.punk, p: up })
        }
    }
}

impl<T> Clone for Com<T> {
    fn clone(&self) -> Self {
        unsafe { (*self.punk).AddRef(); }
        Com { punk: self.punk, p: self.punk as *mut T }
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe { (*self.punk).Release(); }
        self.punk = source.punk;
        unsafe { (*self.punk).AddRef(); }
    }
}

impl<T> Drop for Com<T> {
    fn drop(&mut self) {
        if self.p != null_mut() {
            unsafe { (*self.punk).Release(); }
        }
        self.p = null_mut();
    }
}

impl<T> Into<*mut T> for Com<T> {
    fn into(self) -> *mut T {
        self.p
    }
}

impl<T> ops::Deref for Com<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.p }
    }
}
impl<T> ops::DerefMut for Com<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.p }
    }
}


extern "system" {
	fn D2D1CreateFactory(
        factoryType: D2D1_FACTORY_TYPE,
		riid: REFIID, 
		pFactoryOptions: *const D2D1_FACTORY_OPTIONS,
        ppIFactory: *mut *mut ID2D1Factory
    ) -> HRESULT;
}

extern "system" {
    pub fn SetProcessDpiAwareness(value: DWORD) -> HRESULT;
}

pub type Factory = Com<ID2D1Factory>;

impl Factory {
    pub fn new() -> Result<Com<ID2D1Factory>, HResultError> {
        let null_opts: *const D2D1_FACTORY_OPTIONS = null();
        let mut fac: *mut ID2D1Factory = null_mut();
        unsafe {
            D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, &ID2D1Factory::uuidof(), null_opts, &mut fac).into_result(|| Com::from_ptr(fac))
        }
    }
}

pub type Brush = Com<ID2D1Brush>;

pub type WindowRenderTarget = Com<ID2D1HwndRenderTarget>;

use winit::Window;
use winit::os::windows::WindowExt;

impl WindowRenderTarget {
    pub fn new(fct: Factory, win: &Window) -> Result<WindowRenderTarget, HResultError> {
        let rc = win.get_inner_size().ok_or(HResultError::new(E_FAIL))?;
        let size = D2D_SIZE_U { width: rc.0, height: rc.1 };
        let pxfmt = D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT_B8G8R8A8_UNORM,
            alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED
        };
        let render_props = D2D1_RENDER_TARGET_PROPERTIES {
            _type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: pxfmt,
            dpiX: 0.0, dpiY: 0.0,
            usage: D2D1_RENDER_TARGET_USAGE_NONE,
            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
        };
        let hwnd_rp = D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd: win.get_hwnd() as HWND,
            pixelSize: size,
            presentOptions: D2D1_PRESENT_OPTIONS_NONE
        };

        let mut hrt: *mut ID2D1HwndRenderTarget = null_mut();
        unsafe {
            fct.CreateHwndRenderTarget(&render_props, &hwnd_rp, &mut hrt).into_result(|| Com::from_ptr(transmute(hrt)))
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        let rs = D2D_SIZE_U { width: w, height: h };
        unsafe { self.Resize(&rs); }
    }
}


impl Brush {
    pub fn solid_color(rt: &WindowRenderTarget, col: D2D1_COLOR_F) -> Result<Brush, HResultError> {
        unsafe {
            let mut brsh: *mut ID2D1SolidColorBrush = null_mut();
            (*rt.p).CreateSolidColorBrush(&col, null_mut(), &mut brsh).into_result(|| Com::from_ptr(transmute(brsh)))
        }
    }

    pub unsafe fn set_color(&self, col: D2D1_COLOR_F) {
        let b: *mut ID2D1SolidColorBrush = transmute(self.p);
        (*b).SetColor(&col);
    }
}

pub type TextFactory = Com<IDWriteFactory>;

//"b859ee5a-d838-4b5b-a2e8-1adc.7d93db48"
const UuidOfIDWriteFactory: IID = GUID { Data1: 0xb859ee5a, Data2: 0xd838, Data3: 0x4b5b, Data4: [0xa2,0xe8,0x1a,0xdc,0x7d,0x93,0xdb,0x48] }; 
extern "system" {
    fn DWriteCreateFactory(factoryType: DWRITE_FACTORY_TYPE, iid: REFIID, factory: *mut *mut IUnknown) -> HRESULT;
}

impl TextFactory {
    pub fn new() -> Result<TextFactory, HResultError> {
        unsafe {
            let fac : *mut IDWriteFactory = uninitialized();
            DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, &UuidOfIDWriteFactory, transmute(&fac)).into_result(|| Com::from_ptr(transmute(fac)))
        }
    }
}
