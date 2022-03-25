use crate::drawable::{DrawOptions, Drawable};
use crate::Renderer;

use std::path::Path;

use uuid::Uuid;

use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

const SQUARE_SIZE: u32 = 30; // In pixels squared

pub struct Document {
    position: (i32, i32),
    page_size: (u32, u32), // In number of squares, 30 x 42
    square_size: u32,
    texture_id: Uuid,
}

impl Document {
    // TODO: allow for different colors every 5 squares
    fn create_surface(page_size: (u32, u32), sheet_path: &Path) -> Result<Surface, String> {
        let src = Surface::from_file(sheet_path)?;
        let mut surface = Surface::new(
            (SQUARE_SIZE + 1) * page_size.0,
            (SQUARE_SIZE + 1) * page_size.1,
            src.pixel_format_enum(),
        )?;

        for i in 0..(page_size.1 as i32) {
            for j in 0..(page_size.0 as i32) {
                let rect_size = if j != page_size.0 as i32 - 1 && i != page_size.1 as i32 - 1 {
                    (SQUARE_SIZE + 1, SQUARE_SIZE + 1)
                } else if i != page_size.1 as i32 - 1 {
                    (SQUARE_SIZE, SQUARE_SIZE + 1)
                } else if j != page_size.1 as i32 - 1 {
                    (SQUARE_SIZE + 1, SQUARE_SIZE)
                } else {
                    (SQUARE_SIZE, SQUARE_SIZE)
                };
                src.blit(
                    Rect::new(0, 0, rect_size.0, rect_size.1),
                    &mut surface,
                    Rect::new(
                        j * (SQUARE_SIZE as i32 + 1) + 1,
                        i * (SQUARE_SIZE as i32 + 1) + 1,
                        rect_size.0,
                        rect_size.1,
                    ),
                )?;
            }
        }

        Ok(surface)
    }

    pub fn new<'r>(
        page_size: (u32, u32),
        sheet_path: &Path,
        renderer: &mut Renderer,
    ) -> Result<Document, String> {
        let x = ((renderer.dimensions().0 / 2) - (page_size.0 * SQUARE_SIZE / 2)) as i32;
        let surface = Document::create_surface(page_size, sheet_path)?;
        let texture_id = renderer.create_texture(&surface)?;

        Ok(Document {
            position: (x, 0),
            page_size,
            square_size: SQUARE_SIZE,
            texture_id,
        })
    }

    pub fn width(&self) -> u32 {
        self.page_size.0 * (self.square_size + 1) - 1
    }

    pub fn height(&self) -> u32 {
        self.page_size.1 * (self.square_size + 1) - 1
    }
}

impl Drawable for Document {
    fn draw<'r>(&self, renderer: &mut Renderer) -> Result<(), String> {
        renderer.draw_fill_rect(
            Rect::new(
                self.position.0 - 3,
                self.position.1 - 3,
                self.width() + 6,
                self.height() + 6,
            ),
            Color::GRAY,
        )?;

        let options = DrawOptions {
            src: None,
            dst: Some(Rect::new(
                self.position.0,
                self.position.1,
                self.width(),
                self.height(),
            )),
            rotation: None,
            flip_h: false,
            flip_v: false,
        };

        renderer.draw_texture(self.texture_id, options)
    }
}
