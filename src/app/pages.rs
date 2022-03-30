use crate::drawable::{DrawOptions, Drawable};
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use uuid::Uuid;

use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

const SQUARE_SIZE: u32 = 30; // In pixels squared

pub struct Pages<'p> {
    pub id: Uuid,
    position: (i32, i32),
    page_size: (u32, u32), // In number of squares, 30 x 42
    square_size: u32,
    renderer: Rc<RefCell<Renderer<'p, 'p>>>,
}

impl<'p> Pages<'p> {
    // Create the page surface given a sheet image and a page size
    fn create_surface(page_size: (u32, u32), image_path: &Path) -> Result<Surface, String> {
        let src = Surface::from_file(image_path)?;
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
        image_path: &Path,
        renderer: Rc<RefCell<Renderer<'p, 'p>>>,
    ) -> Result<Pages<'p>, String> {
        let x = ((renderer.borrow().dimensions().0 / 2) - (page_size.0 * SQUARE_SIZE / 2)) as i32;
        let surface = Pages::create_surface(page_size, image_path)?;
        let id = Uuid::new_v4();

        renderer.borrow_mut().create_texture(id, vec![&surface])?;

        Ok(Pages {
            position: (x, 0),
            page_size,
            square_size: SQUARE_SIZE,
            id,
            renderer,
        })
    }

    pub fn width(&self) -> u32 {
        self.page_size.0 * (self.square_size + 1) - 1
    }

    pub fn height(&self) -> u32 {
        self.page_size.1 * (self.square_size + 1) - 1
    }

    pub fn change_page_style(&self, image_path: &Path) -> Result<(), String> {
        let surface = Pages::create_surface(self.page_size, image_path)?;

        self.renderer
            .borrow_mut()
            .create_texture(self.id, vec![&surface])?;

        Ok(())
    }
}

impl<'p> Drawable for Pages<'p> {
    fn draw(&self, renderer: Rc<RefCell<Renderer>>) -> Result<(), String> {
        renderer.borrow_mut().draw_fill_rect(
            Rect::new(
                self.position.0 - 3,
                self.position.1 - 3,
                self.width() + 6,
                self.height() + 6,
            ),
            Color::GRAY,
            true,
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
            on_world: true,
        };

        renderer.borrow_mut().draw_texture(self.id, 0, options)
    }
}
