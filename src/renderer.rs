use crate::drawable::DrawOptions;
use crate::position::Position;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::ttf::{Font, FontStyle};
use sdl2::video::WindowContext;

use std::collections::HashMap;

use uuid::Uuid;

pub struct Renderer<'c, 'tc, 'ttf> {
    canvas: &'c mut WindowCanvas,
    tex_creator: &'tc TextureCreator<WindowContext>,
    textures: HashMap<Uuid, Vec<Texture<'tc>>>,
    fonts: HashMap<String, Font<'ttf, 'ttf>>,
    camera: Rect,
    scroll_max: i32,
}

impl<'c, 'tc, 'ttf> Renderer<'c, 'tc, 'ttf> {
    // Only one Renderer will ever be constructed. The default settings set the window to be
    // maximized to the default display, allowing the window to be resized.
    pub(super) fn new(
        canvas: &'c mut WindowCanvas,
        tex_creator: &'tc TextureCreator<WindowContext>,
        fonts: HashMap<String, Font<'ttf, 'ttf>>,
    ) -> Renderer<'c, 'tc, 'ttf> {
        let camera = Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1);

        Renderer {
            canvas,
            tex_creator,
            textures: HashMap::new(),
            fonts,
            camera,
            scroll_max: 0,
        }
    }

    pub(crate) fn create_textures(
        &mut self,
        id: Uuid,
        surfaces: Vec<&Surface>,
    ) -> Result<(), String> {
        let mut textures = Vec::new();
        for surface in surfaces {
            let texture =
                Texture::from_surface(surface, &self.tex_creator).map_err(|e| e.to_string())?;

            textures.push(texture);
        }

        self.textures.insert(id, textures);

        Ok(())
    }

    pub(crate) fn create_text(
        &mut self,
        id: Uuid,
        text: String,
        font_name: String,
        font_style: FontStyle,
        point: u16,
        color: Color,
    ) -> Result<(), String> {
        let mut font_name = font_name;
        if font_style.bits() & 1 == 1 {
            font_name.push_str("-Bold");
            if font_style.bits() & 2 == 2 {
                font_name.push_str("Italic");
            }
        } else if font_style.bits() & 2 == 2 {
            font_name.push_str("-Italic");
        }
        font_name.push('_');
        font_name.push_str(&point.to_string());

        match self.fonts.get(&font_name) {
            Some(font) => {
                let text_surface = font
                    .render(&text)
                    .blended(color)
                    .map_err(|e| e.to_string())?;
                let texture = Texture::from_surface(&text_surface, &self.tex_creator)
                    .map_err(|e| e.to_string())?;

                self.textures.insert(id, vec![texture]);
            }
            None => return Err("Error retrieving font.".to_string()),
        }

        Ok(())
    }

    // Clears canvas
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();
    }

    // Applies any updates to the Renderer's canvas.
    pub fn update(&mut self) {
        self.canvas.present();
    }

    // Returns the dimensions of the window.
    pub fn dimensions(&self) -> (u32, u32) {
        self.canvas.window().size()
    }

    pub fn camera(&self) -> Rect {
        self.camera
    }

    pub fn set_camera(&mut self, rect: Rect) {
        self.camera = rect
    }

    // scrolls camera by dy amount
    pub fn scroll(&mut self, dy: i32) {
        // Keep the scrolling within the pages
        let new_y = ((self.camera.y - dy * 62).max(0)).min(self.scroll_max);

        self.camera = Rect::new(
            self.camera.x,
            new_y,
            self.camera.width(),
            self.camera.height(),
        )
    }

    pub fn set_scroll_max(&mut self, new_max: i32) {
        self.scroll_max = new_max
    }

    // Draws a texture associated with an object_id and the index into its texture Vec
    pub fn draw_texture(
        &mut self,
        object_id: Uuid,
        index: usize,
        options: DrawOptions,
    ) -> Result<(), String> {
        let textures;
        match self.textures.get(&object_id) {
            Some(t) => textures = t,
            None => return Err("Texture not found".to_string()),
        }

        // Convert from all positions to screen coordinates
        let position = options
            .position
            .to_free_on_screen(Some(self.dimensions()), Some(self.camera))?;

        self.canvas.copy_ex(
            &textures[index],
            options.src,
            Rect::new(position.x(), position.y(), options.size.0, options.size.1),
            match options.rotation {
                Some(rotation) => rotation.0,
                None => 0.0,
            },
            match options.rotation {
                Some(rotation) => rotation.1,
                None => Point::new(0, 0),
            },
            options.flip_h,
            options.flip_v,
        )?;

        Ok(())
    }

    pub fn draw_fill_rect(
        &mut self,
        position: Position,
        size: (u32, u32),
        color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);

        let position = position.to_free_on_screen(Some(self.dimensions()), Some(self.camera))?;

        self.canvas
            .fill_rect(Rect::new(position.x(), position.y(), size.0, size.1))
    }

    pub fn draw_rect(
        &mut self,
        position: Position,
        thickness: i32,
        size: (u32, u32),
        color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);

        let mut position =
            position.to_free_on_screen(Some(self.dimensions()), Some(self.camera))?;
        let mut size = size;

        // Since cannot set thickness of rectangle use canvas.draw_fill_rect
        // This instead draws <thickness> concentric rectangles outwards
        for _ in 0..thickness {
            self.canvas
                .draw_rect(Rect::new(position.x(), position.y(), size.0, size.1))?;
            position = Position::add(position, -1, -1);
            size = (size.0 + 2, size.1 + 2);
        }

        Ok(())
    }
}
