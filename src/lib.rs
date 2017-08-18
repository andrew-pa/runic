use std::error::Error;

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

#[derive(Debug,Copy,Clone)]
pub enum MouseButton {
    Left, Right, Middle
}

/// A key on the keyboard
///
/// There is some overlap between some of these variants, it is likely that your App will receive
/// multiple events with each version
#[derive(Debug,Copy,Clone)]
pub enum KeyCode {
    Unknown,
    /// Characters as processed by the operating system
    Character(char),
    /// Characters as printed on the keycaps, no processing
    RawCharacter(char), //characters as printed on the keycaps
    Left, Right, Up, Down,
    Backspace, Enter, Escape, Ctrl, Delete,
    /// Function keys, numbered like keys, starting at 1 = F1
    Function(u8)
}

/// An event from the system
#[derive(Copy,Clone,Debug)]
pub enum Event {
    /// The window has resized
    ///
    /// The first two u32s are the width and height in pixels, and the Point is the width/height in
    /// device independent units
    Resize(u32, u32, Point),

    /// The mouse has moved, with an optional mouse down. Coords in DIPs
    MouseMove(Point, Option<MouseButton>),
    /// A mouse button has been depressed. Coords in DIPs
    MouseDown(Point, MouseButton),
    /// A mouse button has been released. Coords in DIPs
    MouseUp(Point, MouseButton),
    /// A key event. True represents pressed, false released
    Key(KeyCode, bool),
}

/// The App trait represents the client rendering and event handling code to the Window
pub trait App {
    /// Re-render the app contents/interface
    fn paint(&mut self, rx: &mut RenderContext);
    /// Handle an event
    fn event(&mut self, e: Event, win: WindowRef);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows as imp;//::{RenderContext, Window, Font, TextLayout};

pub struct Window(imp::Window);

#[derive(Clone)]
/// Represents a (weak) reference to a window so that it can be notified of events by the app
pub struct WindowRef(imp::WindowRef);

pub struct Font(imp::Font);
pub struct TextLayout(imp::TextLayout);
pub struct RenderContext(imp::RenderContext);

impl Window {
    /// Create a new window, then create an App trait object with the render context associated
    /// with it
    pub fn new<A: App + 'static, F: FnOnce(&mut RenderContext)->A>(title: &str, width: usize, height: usize, appf: F) 
        -> Result<Self, Box<Error>> 
    {
        imp::Window::new(title, width, height, appf).map(Window)
    }

    /// Run the message loop for this window. This function doesn't return until the window exits
    pub fn show(&mut self) { self.0.show(); }
}

impl WindowRef {
    /// Request the window to quit. This will cause `Window::show` to return
    pub fn quit(&self) { self.0.quit() }
}

impl Font {
    /// Create a new font, looking the name up in the system font registery
    pub fn new(rx: &mut RenderContext, name: &str, size: f32, weight: FontWeight, style: FontStyle) -> Result<Font, Box<Error>> {
        imp::Font::new(&mut rx.0, name, size, weight, style).map(Font)
    }
}

impl TextLayout {
    /// Create a new text layout. The text will be wrapped to `width` and `height`
    pub fn new(rx: &mut RenderContext, text: &str, f: &Font, width: f32, height: f32) -> Result<TextLayout, Box<Error>> {
        imp::TextLayout::new(&mut rx.0, text, &f.0, width, height).map(TextLayout)
    }
    
    /// Calculate the bounding rectangle of this text layout
    pub fn bounds(&self) -> Rect { self.0.bounds() }

    /// Calculate the bounding rectangle of the character at `index`
    pub fn char_bounds(&self, index: usize) -> Rect { self.0.char_bounds(index) }
}

impl RenderContext {
    /// Clear the window
    pub fn clear(&mut self, col: Color) { self.0.clear(col); }

    /// Draw a rectangle, only the outline
    pub fn stroke_rect(&mut self, rect: Rect, col: Color, stroke_width: f32) { self.0.stroke_rect(rect, col, stroke_width); }

    /// Draw a filled rectangle
    pub fn fill_rect(&mut self, rect: Rect, col: Color) { self.0.fill_rect(rect, col); }

    /// Draw a line
    pub  fn draw_line(&mut self, a: Point, b: Point, col: Color, stroke_width: f32) { self.0.draw_line(a,b,col,stroke_width); }

    /// Draw text, wrapped within `rect`
    ///
    /// This function is best for dynamic text, that won't need to be measured
    pub fn draw_text(&mut self, rect: Rect, s: &str, col: Color, f: &Font) { self.0.draw_text(rect,s,col,&f.0); }

    /// Draw a text layout
    ///
    /// This is ideal for text that needs to be measured or layed out but doesn't change as
    /// frequently
    pub fn draw_text_layout(&mut self, p: Point, txl: &TextLayout, col: Color) { self.0.draw_text_layout(p,&txl.0,col); }

    /// Translate the origin point that primitives will be drawn relative to
    ///
    /// Default value is (0,0)
    pub fn translate(&mut self, p: Point) { self.0.translate(p); }

    /// Calculate the size of the area being rendered into
    pub fn bounds(&self) -> Rect { self.0.bounds() }
}


// if I want to do this well:
// Split RenderContext and Window apart into seperate modules
// Write Direct2D and Cairo/Pango RenderContext modules
// Write Win32, Cocoa, GTK, etc... Window modules
// Provide a system to choose between them
// ✓ Define passthrough structs to prove documentation targets and a unified single spec of APIs
