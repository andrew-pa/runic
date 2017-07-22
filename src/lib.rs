use std::error::Error;

#[derive(Copy,Clone,Debug)]
pub struct Point { pub x: f32, pub y: f32 }

impl Point {
    pub fn xy(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Rect {
    x: f32, y: f32, w: f32, h: f32
}

impl Rect {
    pub fn xywh(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn pnwh(p: Point, w: f32, h: f32) -> Rect {
        Rect { x: p.x, y: p.y, w, h }
    }
    pub fn offset(&self, p: Point) -> Rect {
        Rect { x: self.x + p.x, y: self.y + p.y, ..*self }
    }
}

#[derive(Copy,Clone,Debug)]
pub struct Color {
    r: f32, g: f32, b: f32, a: f32
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
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

#[derive(Debug)]
pub enum KeyCode {
}

#[derive(Debug)]
pub enum Event {
    Resize(u32,u32),
    MouseMove(Point, Option<MouseButton>),
    MouseDown(Point, MouseButton),
    MouseUp(Point, MouseButton),
    Key(KeyCode, bool),
    KeyChar(char, bool)
}
pub trait App {
    fn init(&mut self, rx: &mut RenderContext) { }
    fn paint(&self, rx: &mut RenderContext);
    fn event(&mut self, e: Event);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{RenderContext, Window, Font, TextLayout};
