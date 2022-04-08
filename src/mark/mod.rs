pub mod textbox;

use crate::drawable::Drawable;
use crate::renderer::Renderer;

pub trait Mark {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String>;
}

impl Drawable for dyn Mark {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        self.draw(renderer)
    }
}
