use crate::drawable::{DrawOptions, Drawable};
use crate::position::Position;
use crate::renderer::Renderer;

use std::path::Path;

use uuid::Uuid;

use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

const SQUARE_SIZE: u32 = 30; // In pixels squared
pub const PAGE_PADDING: i32 = 200;

#[derive(Clone, Copy)]
pub enum PageStyle {
    WhiteSquared = 0,
    WhitePlain = 1,
    BeigeSquared = 2,
    BeigePlain = 3,
}

impl PageStyle {
    pub fn path(&self) -> &Path {
        Path::new(match *self {
            PageStyle::WhiteSquared => "assets/white_squared.png",
            PageStyle::WhitePlain => "assets/white_plain.png",
            PageStyle::BeigeSquared => "assets/beige_squared.png",
            PageStyle::BeigePlain => "assets/beige_plain.png",
        })
    }
}

pub struct Pages {
    pub id: Uuid,
    position: Position,
    page_size: (u32, u32), // In number of squares, 30 x 42
    pages: u32,
    square_size: u32,
    style: PageStyle,
}

impl Pages {
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

    pub fn new(page_size: (u32, u32), renderer: &mut Renderer) -> Result<Pages, String> {
        let id = Uuid::new_v4();

        // Create all the page style textures to switch between them
        let white_squared_sfc = Pages::create_surface(page_size, PageStyle::WhiteSquared.path())?;
        let white_plain_sfc = Pages::create_surface(page_size, PageStyle::WhitePlain.path())?;
        let beige_squared_sfc = Pages::create_surface(page_size, PageStyle::BeigeSquared.path())?;
        let beige_plain_sfc = Pages::create_surface(page_size, PageStyle::BeigePlain.path())?;

        renderer.create_texture(
            id,
            vec![
                &white_squared_sfc,
                &white_plain_sfc,
                &beige_squared_sfc,
                &beige_plain_sfc,
            ],
        )?;

        Ok(Pages {
            position: Position::FreeOnWorld(0, PAGE_PADDING),
            page_size,
            square_size: SQUARE_SIZE,
            id,
            pages: 1,
            style: PageStyle::WhiteSquared,
        })
    }

    pub fn square_size(&self) -> u32 {
        self.square_size
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn page_width(&self) -> u32 {
        self.page_size.0 * (self.square_size + 1) - 1
    }

    pub fn page_height(&self) -> u32 {
        self.page_size.1 * (self.square_size + 1) - 1
    }

    pub fn total_height(&self) -> u32 {
        self.pages * self.page_height() + PAGE_PADDING as u32 * (self.pages - 1)
    }

    pub fn style(&self) -> PageStyle {
        self.style
    }

    pub fn pages(&self) -> u32 {
        self.pages
    }

    pub fn set_style(&mut self, style: PageStyle) {
        self.style = style
    }

    pub fn add_page(&mut self) {
        self.pages += 1
    }

    pub fn remove_page(&mut self) {
        self.pages = 1.max(self.pages - 1)
    }

    // Get the FreeOnWorld position of the 0 indexed page
    pub fn get_page_position(&self, page_num: u32) -> Position {
        Position::add(
            self.position(),
            0,
            (self.page_height() as i32 + PAGE_PADDING) * page_num as i32,
        )
    }

    // Returns the 0 indexed page in which point give is located in on screen
    // If it is outside any page, returns None
    pub fn page_contains(&self, point: Position, camera: Rect) -> Option<u32> {
        // Could be made more efficient without a for loop
        for i in 0..self.pages {
            let p = self
                .get_page_position(i)
                .to_free_on_screen(None, Some(camera))
                .unwrap();

            let rect = Rect::new(p.x(), p.y(), self.page_width(), self.page_height());

            if rect.contains_point(point) {
                return Some(i);
            }
        }

        None
    }
}

impl Drawable for Pages {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        // Set maximum height scrollable depending on pages height
        renderer.set_scroll_max(
            (self.total_height() + 2 * PAGE_PADDING as u32 - renderer.dimensions().1) as i32,
        );

        for i in 0..(self.pages as i32) {
            // Draw outline
            renderer.draw_fill_rect(
                Position::FreeOnWorld(
                    self.position.x() - 3,
                    self.position.y() + (self.page_height() as i32 + PAGE_PADDING) * i - 3,
                ),
                (self.page_width() + 6, self.page_height() + 6),
                Color::GRAY,
            )?;

            let options = DrawOptions {
                src: None,
                position: Position::FreeOnWorld(
                    self.position.x(),
                    self.position.y() + (self.page_height() as i32 + PAGE_PADDING) * i,
                ),
                size: (self.page_width(), self.page_height()),
                rotation: None,
                flip_h: false,
                flip_v: false,
            };

            renderer.draw_texture(self.id, self.style as usize, options)?;
        }

        Ok(())
    }
}
