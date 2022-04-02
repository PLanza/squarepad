use crate::position::Position;
use crate::renderer::Renderer;

use sdl2::rect::{Point, Rect};

pub trait Drawable {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String>;
}

#[derive(Debug)]
pub struct DrawOptions {
    pub src: Option<Rect>,
    pub position: Position,
    pub size: (u32, u32),
    pub rotation: Option<(f64, Point)>,
    pub flip_h: bool,
    pub flip_v: bool,
}
