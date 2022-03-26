use crate::drawable::{DrawOptions, Drawable};
use crate::renderer::Renderer;

use std::path::Path;

use uuid::Uuid;

use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

const SQUARE_SIZE: u32 = 30; // In pixels squared

pub struct Pages {
    pub id: Uuid,
    position: (i32, i32),
    page_size: (u32, u32), // In number of squares, 30 x 42
    square_size: u32,
}

impl Pages {
    // Create the page surface given a sheet image and a page size
    fn create_surface(page_size: (u32, u32), sheet_path: &Path) -> Result<Surface, String> {
        let src = Surface::from_file(sheet_path)?;
        let mut surface = Surface::new(
            (SQUARE_SIZE + 1) * page_size.0 - 1,
            (SQUARE_SIZE + 1) * page_size.1 - 1,
            src.pixel_format_enum(),
        )?;

        let mut i = 0;
        while i < page_size.1 as i32 {
            // Change clip height if near the edge
            let h = if i <= (page_size.1 - 5) as i32 {
                5 * (SQUARE_SIZE + 1)
            } else {
                (page_size.1 % 5) * (SQUARE_SIZE + 1) - 1
            };

            let mut j = 0;
            while j < page_size.0 as i32 {
                // Change clip width if near the edge
                let w = if j <= (page_size.0 - 5) as i32 {
                    5 * (SQUARE_SIZE + 1)
                } else {
                    (page_size.0 % 5) * (SQUARE_SIZE + 1) - 1
                };

                src.blit(
                    Rect::new(0, 0, w, h),
                    &mut surface,
                    Rect::new(
                        j * (SQUARE_SIZE as i32 + 1),
                        i * (SQUARE_SIZE as i32 + 1),
                        w,
                        h,
                    ),
                )?;

                j += 5;
            }
            i += 5;
        }

        Ok(surface)
    }

    pub fn new<'r>(
        page_size: (u32, u32),
        sheet_path: &Path,
        renderer: &mut Renderer,
    ) -> Result<Pages, String> {
        let x = ((renderer.dimensions().0 / 2) - (page_size.0 * SQUARE_SIZE / 2)) as i32;
        let surface = Pages::create_surface(page_size, sheet_path)?;
        let id = renderer.create_texture(&surface)?;

        Ok(Pages {
            position: (x, 0),
            page_size,
            square_size: SQUARE_SIZE,
            id,
        })
    }

    pub fn width(&self) -> u32 {
        self.page_size.0 * (self.square_size + 1) - 1
    }

    pub fn height(&self) -> u32 {
        self.page_size.1 * (self.square_size + 1) - 1
    }
}

impl Drawable for Pages {
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

        renderer.draw_texture(self.id, options)
    }
}
