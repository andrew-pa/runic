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
    fn new(win: &mut winit::window::Window) -> Result<Self, Box<dyn Error>> where Self: Sized;

    /// Create a new font, looking the name up in the system font registery
    fn new_font(&self, name: &str, size: f32, weight: FontWeight, style: FontStyle) -> Result<Font, Box<dyn Error>>;

    /// Create a new text layout. The text will be wrapped to `width` and `height`
    fn new_text_layout(&self, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<dyn Error>>;

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

pub use winit::event::{MouseButton, VirtualKeyCode, ElementState, TouchPhase, ModifiersState, KeyboardInput};
pub use winit::dpi as dpi;
pub use winit::event::WindowEvent as Event;
pub use winit::window::Window as Window;
pub use winit::window::WindowBuilder as WindowOptions;

/// The App trait represents an application that uses RenderContext to draw its interface.
/// The `run` function is provided to conveniently set up the loop that handles winit events and
/// redraws the App interface using `paint`
pub trait App {
    /// Initialize a new App, this automatically called by `runic::start()` after initializing the
    /// system and creating a RenderContext
    fn init(rx: &mut RenderContext) -> Self;

    /// do any additional necessary configuration to the application window
    fn configure_window(&mut self, _: &mut Window) {}

    /// Draw the interface for the App, this is called enough for animations and such
    fn paint(&mut self, rx: &mut RenderContext);

    /// Handle any events this App recieves. `Event` is an alias for `winit::event::WindowEvent`,
    /// device events aren't passed to applications. Return false to exit the event loop and exit
    /// the application
    fn event(&mut self, e: Event) -> bool;
}

/// Start an runic app specified by `AppT` and run the event loop
/// the WindowOptions will be used to create the window the app will run in
pub fn start<AppT: 'static + App>(winopts: WindowOptions) -> ! {
    imp::init();
    let el = winit::event_loop::EventLoop::new();
    let mut window = winopts.build(&el).expect("create new window"); 
    let mut rx = RenderContext::new(&mut window).expect("create render context");
    let mut app = AppT::init(&mut rx);
    app.configure_window(&mut window);
    el.run(move |ev, _, ctrl_flow| {
        use winit::event::Event;
        use winit::event_loop::ControlFlow;
        *ctrl_flow = ControlFlow::Poll;
        match ev {
            Event::WindowEvent { event, .. } => {
                #[allow(deprecated)]
                match event {
                    winit::event::WindowEvent::CursorMoved { device_id, position, modifiers } =>  {
                        let scaled = rx.pixels_to_points(Point { x: position.x as f32, y: position.y as f32 });
                        if app.event(winit::event::WindowEvent::CursorMoved {
                            device_id, position: dpi::PhysicalPosition{ x: scaled.x as f64, y: scaled.y as f64 }, modifiers 
                        }) {
                            *ctrl_flow = ControlFlow::Exit;
                        }
                    },
                    winit::event::WindowEvent::Resized(size) => {
                        rx.resize(size.width, size.height);
                        window.request_redraw();
                        if app.event(event) { *ctrl_flow = ControlFlow::Exit }
                    },
                    _=> {
                        if app.event(event) {
                            *ctrl_flow = ControlFlow::Exit
                        }
                    }
                }
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                rx.start_paint();
                app.paint(&mut rx);
                rx.end_paint();
            }
            _ => ()
        }
    })
}
