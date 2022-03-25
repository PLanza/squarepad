use crate::drawable::DrawOptions;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use std::collections::HashMap;

use uuid::Uuid;

pub struct Renderer<'c, 't> {
    canvas: &'c mut WindowCanvas,
    tex_creator: &'t TextureCreator<WindowContext>,
    textures: HashMap<Uuid, Texture<'t>>,
}

impl<'c, 't> Renderer<'c, 't> {
    // Only one Renderer will ever be constructed. The default settings set the window to be
    // maximized to the default display, allowing the window to be resized.
    pub(super) fn new(
        canvas: &'c mut WindowCanvas,
        tex_creator: &'t TextureCreator<WindowContext>,
    ) -> Renderer<'c, 't> {
        Renderer {
            canvas,
            tex_creator,
            textures: HashMap::new(),
        }
    }

    pub(crate) fn create_texture(&mut self, surface: &Surface) -> Result<Uuid, String> {
        let texture =
            Texture::from_surface(surface, &self.tex_creator).map_err(|e| e.to_string())?;
        let id = Uuid::new_v4();

        self.textures.insert(id, texture);

        Ok(id)
    }

    pub fn draw_texture(&mut self, texture_id: Uuid, options: DrawOptions) -> Result<(), String> {
        let texture;
        match self.textures.get(&texture_id) {
            Some(t) => texture = t,
            None => return Err("Texture not found".to_string()),
        }

        self.canvas.copy_ex(
            texture,
            options.src,
            options.dst,
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

    // For testing purposes
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
}
