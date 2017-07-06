use std::error::Error;

pub struct Point { x: f32, y: f32 }

impl Point {
    pub fn xy(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

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
pub trait App {
    fn paint(&self, rx: &mut RenderContext);
    fn event(&mut self, e: Event);
}

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::{RenderContext, Window};
