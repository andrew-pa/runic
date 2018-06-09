extern crate winit;
use std::error::Error;
use std::ops::Range;

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

#[cfg(linux)]
extern crate x11_dl;

#[cfg(windows)]
mod windows;
#[cfg(target_os="macos")]
mod macos;
#[cfg(linux)]
mod unix;

#[cfg(any(target_os="macos", target_os="linux"))]
mod cairo_context;

#[cfg(windows)]
use windows as imp;
#[cfg(target_os="macos")]
use macos as imp;
#[cfg(linux)]
use unix as imp;

#[derive(Copy,Clone,Debug)]
pub struct Point { pub x: f32, pub y: f32 }

impl Point {
    pub fn xy(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn x(x: f32) -> Point {
        Point { x, y: 0.0 }
    }
    pub fn y(y: f32) -> Point {
        Point { x: 0.0, y }
    }
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}
impl<'a> std::ops::Add for &'a Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
impl<'a> std::ops::Sub for &'a Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
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

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Point {
        Point {x, y}
    }
}

impl Default for Point {
    fn default() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

// should the whole API shift to f64??
// at least Point should probably be Point<T>. Rect would also probably be enhanced
// there is probably a crate with this stuff in it
impl From<(f64, f64)> for Point {
    fn from((x, y): (f64, f64)) -> Point {
        Point {x: x as f32, y: y as f32}
    }
}
#[derive(Copy,Clone,Debug)]
pub struct Rect {
    pub x: f32, pub y: f32, pub w: f32, pub h: f32
}

impl Rect {
    /// Create a rectangle at the origin with given extents
    pub fn wh(w: f32, h: f32) -> Rect {
        Rect { x: 0.0, y: 0.0, w, h }
    }

    /// Create a rectangle from coordinates and extents
    pub fn xywh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }
    /// Create a rectangle from a point and extents
    pub fn pnwh(p: Point, w: f32, h: f32) -> Rect {
        Rect { x: p.x, y: p.y, w, h }
    }
    /// Create a rectangle from a point and a 'point' containing the extents
    pub fn from_points(p: Point, size: Point) -> Rect {
        Rect { x: p.x, y: p.y, w: size.x, h: size.y }
    }
    /// Returns a new rectangle the same size as this one but offset by `p`
    pub fn offset(&self, p: Point) -> Rect {
        Rect { x: self.x + p.x, y: self.y + p.y, ..*self }
    }
    /// Returns whether `p` is contained in this rectangle
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

    pub fn black() -> Color { Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 } }
    pub fn white() -> Color { Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 } }

    /// Linearly interpolate between two colors
    pub fn mix(&self, other: Color, t: f32) -> Color {
        let omt = 1.0 - t;
        Color {
            r: self.r*omt + other.r*t,
            g: self.g*omt + other.g*t,
            b: self.b*omt + other.b*t,
            a: self.a*omt + other.a*t    
        }
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

    /// Calculate the index and bounding rectangle of the character under the point `p`, relative
    /// to the layout's internal coordinate system, given by `bounds()`
    fn hit_test(&self, p: Point) -> Option<(usize, Rect)>;

    fn color_range(&self, rx: &RenderContext, range: Range<u32>, col: Color);
    fn style_range(&self, range: Range<u32>, style: FontStyle);
    fn weight_range(&self, range: Range<u32>, weight: FontWeight);
    fn underline_range(&self, range: Range<u32>, ul: bool);
    fn size_range(&self, range: Range<u32>, size: f32);
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

    /// Convert a point that is in screen pixels to a point that is Device Independent Points.
    /// There are 96 DIPs in an inch
    fn pixels_to_points(&self, p: Point) -> Point;
}

/// The App trait represents an application that uses RenderContext to draw its interface.
/// The `run` function is provided to conveniently set up the loop that handles winit events and
/// redraws the App interface using `paint`
pub trait App {
    fn paint(&mut self, rx: &mut RenderContext);
    fn event(&mut self, e: winit::Event) -> bool;

    fn run(&mut self, rx: &mut RenderContext, evloop: &mut winit::EventsLoop) {
        use winit::*;
        /*let mut running = true;
        while running {
            let mut need_repaint = false;
            evloop.poll_events(|e| {
                match e {
                    Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => { running = false },
                    Event::WindowEvent { event: WindowEvent::Resized(w, h), .. } => {
                        rx.resize(w,h);
                        need_repaint = true;
                        running = !self.event(e);
                    },
                    Event::WindowEvent { event: WindowEvent::CursorMoved { position, device_id, modifiers }, window_id } => {
                        need_repaint = true;
                        let Point {x:a, y:b} = rx.pixels_to_points(position.into());
                        running = !self.event(Event::WindowEvent {
                            event: WindowEvent::CursorMoved {
                                position: (a as f64, b as f64), device_id, modifiers
                            },
                            window_id
                        });
                    },
                    _ => {
                        need_repaint = true;
                        running = !self.event(e);
                    }
                };
            });
            if running && need_repaint {
                rx.start_paint();
                self.paint(rx);
                rx.end_paint();
                ::std::thread::sleep(::std::time::Duration::from_millis(6)); 
            }
        }*/
        evloop.run_forever(|ee| {
            match ee.clone() {
                Event::WindowEvent { event: e, window_id, .. } => {
                    match e {
                        WindowEvent::CloseRequested => ControlFlow::Break,
                        WindowEvent::CursorMoved { position, device_id, modifiers } => {
                            rx.start_paint();
                            self.paint(rx);
                            rx.end_paint();
                            let Point {x:a, y:b} = rx.pixels_to_points(position.into());
                            if self.event(Event::WindowEvent {
                                event: WindowEvent::CursorMoved {
                                    position: (a as f64, b as f64), device_id, modifiers
                                }, window_id
                            }) { ControlFlow::Break } else { ControlFlow::Continue }
                        },
                        WindowEvent::Resized(w, h) => {
                            rx.resize(w,h);
                            rx.start_paint();
                            self.paint(rx);
                            rx.end_paint();
                            if self.event(ee) { ControlFlow::Break } else { ControlFlow::Continue } 
                        },
                        _ => {
                            rx.start_paint();
                            self.paint(rx);
                            rx.end_paint();
                            if self.event(ee) { ControlFlow::Break } else { ControlFlow::Continue }
                        } 
                    }
                },
                _ => ControlFlow::Continue
            }
        });
    }
}

/// Initialize runic library. On Windows this enables HiDPI mode
pub fn init() { imp::init(); }
