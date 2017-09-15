extern crate winit;
use std::error::Error;

#[cfg(target_os="macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os="macos")]
extern crate cocoa;


#[cfg(any(target_os="macos", target_os="linux"))]
extern crate cairo_sys;
#[cfg(any(target_os="macos", target_os="linux"))]
extern crate pango_sys;
#[cfg(any(target_os="macos", target_os="linux"))]
extern crate pangocairo_sys;
#[cfg(any(target_os="macos", target_os="linux"))]
extern crate gobject_sys;


#[cfg(windows)]
mod windows;
#[cfg(target_os="macos")]
mod macos;

#[cfg(any(target_os="macos", target_os="linux"))]
mod cairo_context;


#[cfg(windows)]
use windows as imp;
#[cfg(target_os="macos")]
use macos as imp;

#[derive(Copy,Clone,Debug)]
pub struct Point { pub x: f32, pub y: f32 }

impl Point {
    pub fn xy(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

impl std::ops::Index<u8> for Point {
    type Output = f32;
    fn index(&self, index: u8) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("out of bounds point index")
        }
    }
}

impl std::ops::IndexMut<u8> for Point {
    fn index_mut(&mut self, index: u8) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("out of bounds point index")
        }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Rect {
    pub x: f32, pub y: f32, pub w: f32, pub h: f32
}

impl Rect {
    pub fn xywh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn pnwh(p: Point, w: f32, h: f32) -> Rect {
        Rect { x: p.x, y: p.y, w, h }
    }
    pub fn from_points(p: Point, size: Point) -> Rect {
        Rect { x: p.x, y: p.y, w: size.x, h: size.y }
    }
    pub fn offset(&self, p: Point) -> Rect {
        Rect { x: self.x + p.x, y: self.y + p.y, ..*self }
    }
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.y >= self.y && p.x <= self.x+self.w && p.y <= self.y+self.h
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Color {
    r: f32, g: f32, b: f32, a: f32
}

impl Color {
    /// Create an opaque color
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    /// Create a color with transparency
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }
}

pub enum FontWeight {
    Light, Regular, Bold
}
pub enum FontStyle {
    Normal, Italic
}

pub type Font = imp::Font;
pub type TextLayout = imp::TextLayout;
pub type RenderContext = imp::RenderContext;

pub trait TextLayoutExt {
    /// Calculate the bounding rectangle of this text layout
    fn bounds(&self) -> Rect;

    /// Calculate the bounding rectangle of the character at `index`
    fn char_bounds(&self, index: usize) -> Rect;
}

pub trait RenderContextExt {
    /// Create a new RenderContext, and resize the window to be in DIP units
    fn new(win: &mut winit::Window) -> Result<Self, Box<Error>> where Self: Sized;

    /// Create a new font, looking the name up in the system font registery
    fn new_font(&self, name: &str, size: f32, weight: FontWeight, style: FontStyle) -> Result<Font, Box<Error>>;

    /// Create a new text layout. The text will be wrapped to `width` and `height`
    fn new_text_layout(&self, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<Error>>;

    /// Clear the window
    fn clear(&mut self, col: Color);

    fn set_color(&mut self, col: Color);

    /// Draw a rectangle, only the outline
    fn stroke_rect(&mut self, rect: Rect, stroke_width: f32);

    /// Draw a filled rectangle
    fn fill_rect(&mut self, rect: Rect);

    /// Draw a line
    fn draw_line(&mut self, a: Point, b: Point, stroke_width: f32);

    /// Draw text, wrapped within `rect`
    ///
    /// This function is best for dynamic text, that won't need to be measured
    fn draw_text(&mut self, rect: Rect, s: &str, f: &Font);

    /// Draw a text layout
    ///
    /// This is ideal for text that needs to be measured or layed out but doesn't change as
    /// frequently
    fn draw_text_layout(&mut self, p: Point, txl: &TextLayout);

    /// Translate the origin point that primitives will be drawn relative to
    ///
    /// Default value is (0,0)
    fn translate(&mut self, p: Point);

    /// Calculate the size of the area being rendered into
    fn bounds(&self) -> Rect;

    /// Start the painting process. Must be called before any drawing functions
    fn start_paint(&mut self);
    /// End the painting process. Call after finishing drawing
    fn end_paint(&mut self);

    /// Resize this RenderContext
    fn resize(&mut self, w: u32, h: u32);
}

/// The App trait represents an application that uses RenderContext to draw its interface.
/// The `run` function is provided to conveniently set up the loop that handles winit events and
/// redraws the App interface using `paint`
use std::rc::Rc;
use std::cell::RefCell;
pub trait App {
    fn paint(&mut self, rx: &mut RenderContext);
    fn event(&mut self, e: winit::Event) -> bool;

    fn run(&mut self, rx: &mut RenderContext, evloop: &mut winit::EventsLoop) {
        use winit::*;
        evloop.run_forever(|ee| {
            match ee.clone() {
                Event::WindowEvent { event: e, .. } => {
                    match e {
                        WindowEvent::Closed => ControlFlow::Break,
                        WindowEvent::Resized(w, h) => {
                            rx.resize(w,h);
                            rx.start_paint();
                            self.paint(rx);
                            rx.end_paint();
                            if self.event(ee) { ControlFlow::Break } else { ControlFlow::Continue } 
                        },
                        _ => { if self.event(ee) { ControlFlow::Break } else { ControlFlow::Continue } } 
                        _ => ControlFlow::Continue
                    }
                }
                _ => ControlFlow::Continue
            }
        });
    }
}

/// Initialize runic library. On Windows this enables HiDPI mode
pub fn init() { imp::init(); }
