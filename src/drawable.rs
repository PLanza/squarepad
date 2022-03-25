use crate::Renderer;

use sdl2::rect::{Point, Rect};

pub trait Drawable {
    fn draw(&self, renderer: &Renderer) {}
}

pub struct DrawableOptions {
    pub src: Option<Rect>,
    pub dst: Option<Rect>,
    pub rotation: (f64, Point),
    pub flip_h: bool,
    pub flip_v: bool,
}
