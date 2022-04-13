use super::Mark;
use crate::drawable::DrawOptions;
use crate::position::PageSquare;
use crate::renderer::Renderer;

pub struct TextBox {
    id: uuid::Uuid,
    page_square: PageSquare, // Position on page
    size: (u32, u32),
    text: String,
    font_name: String,
}

impl TextBox {
    pub fn new(page_square: PageSquare, font_name: String) -> TextBox {
        TextBox {
            id: uuid::Uuid::new_v4(),
            page_square,
            size: (0, 0),
            text: "".to_string(),
            font_name,
        }
    }
}

impl Mark for TextBox {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        let options = DrawOptions {
            src: None,
            position: self.page_square.position,
            size: self.size,
            rotation: None,
            flip_h: false,
            flip_v: false,
        };

        renderer.draw_texture(self.id, 0, options)
    }
}
