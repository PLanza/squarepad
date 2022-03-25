use crate::drawable::Drawable;
use crate::Renderer;

use std::path::Path;

use uuid::Uuid;

use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

const SQUARE_SIZE: u32 = 30; // In pixels squared

pub struct Document {
    page_size: (u32, u32), // In number of squares, 30 x 42
    square_size: u32,
    texture_id: Uuid,
}

impl Document {
    pub fn new<'r>(
        page_size: (u32, u32),
        texture_path: &Path,
        renderer: &'r mut Renderer<'r>,
    ) -> Result<Document, String> {
        let surface = Surface::from_file(texture_path)?;
        let texture_id = renderer.create_texture(&surface)?;

        Ok(Document {
            page_size,
            square_size: SQUARE_SIZE,
            texture_id,
        })
    }

    pub fn width(&self) -> u32 {
        self.square_size * self.page_size.0
    }

    pub fn height(&self) -> u32 {
        self.square_size * self.page_size.1
    }
}

impl Drawable for Document {
    fn draw(&self, renderer: &Renderer) {
        
    }
}
