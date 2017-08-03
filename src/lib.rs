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
    pub fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }
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

#[derive(Debug,Copy,Clone)]
pub enum KeyCode {
    Unknown,
    Character(char), //characters as processed from OS
    RawCharacter(char), //characters as printed on the keycaps
    Left, Right, Up, Down,
    Backspace, Enter, Escape, Ctrl,
    Function(u8)
}

#[derive(Copy,Clone,Debug)]
pub enum Event {
    Resize(u32, u32, Point),
    MouseMove(Point, Option<MouseButton>),
    MouseDown(Point, MouseButton),
    MouseUp(Point, MouseButton),
    Key(KeyCode, bool),
}
pub trait App {
    fn paint(&mut self, rx: &mut RenderContext);
    fn event(&mut self, e: Event);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{RenderContext, Window, Font, TextLayout};
