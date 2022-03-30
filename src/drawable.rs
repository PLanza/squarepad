use crate::renderer::Renderer;

use sdl2::rect::{Point, Rect};

use std::cell::RefCell;
use std::rc::Rc;

pub trait Drawable {
    fn draw(&self, renderer: Rc<RefCell<Renderer>>) -> Result<(), String>;
}

pub struct DrawOptions {
    pub src: Option<Rect>,
    pub dst: Option<Rect>,
    pub rotation: Option<(f64, Point)>,
    pub flip_h: bool,
    pub flip_v: bool,
    pub on_world: bool,
}
