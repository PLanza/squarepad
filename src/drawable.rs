use crate::renderer::Renderer;

use sdl2::rect::{Point, Rect};

pub trait Drawable {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String>;
}

pub struct DrawOptions {
    pub src: Option<Rect>,
    pub dst: Option<Rect>,
    pub rotation: Option<(f64, Point)>,
    pub flip_h: bool,
    pub flip_v: bool,
    pub on_world: bool,
}
