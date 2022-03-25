use crate::drawable::DrawableOptions;
use crate::SdlContext;

use sdl2::pixels::Color;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use std::collections::HashMap;

use uuid::Uuid;

pub struct Renderer<'r> {
    canvas: WindowCanvas,
    tex_creator: TextureCreator<WindowContext>,
    textures: HashMap<Uuid, Texture<'r>>,
}

impl<'r> Renderer<'r> {
    // Only one Renderer will ever be constructed. The default settings set the window to be
    // maximized to the default display, allowing the window to be resized.
    pub(super) fn new(sdl_context: &SdlContext) -> Result<Renderer, String> {
        let display_bounds = sdl_context.video_subsystem.display_usable_bounds(0)?;

        let window = sdl_context
            .video_subsystem
            .window("SquarePad", display_bounds.width(), display_bounds.height())
            .maximized()
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let tex_creator = canvas.texture_creator();

        Ok(Renderer {
            canvas,
            tex_creator,
            textures: HashMap::new(),
        })
    }

    pub(crate) fn create_texture(&'r mut self, surface: &Surface) -> Result<Uuid, String> {
        let texture =
            Texture::from_surface(surface, &self.tex_creator).map_err(|e| e.to_string())?;
        let id = Uuid::new_v4();

        self.textures.insert(id, texture);

        Ok(id)
    }

    pub fn draw_texture(
        &'r mut self,
        texture_id: Uuid,
        options: DrawableOptions,
    ) -> Result<(), String> {
        let texture;
        match self.textures.get(&texture_id) {
            Some(t) => texture = t,
            None => return Err("Texture not found".to_string()),
        }

        self.canvas.copy_ex(
            texture,
            options.src,
            options.dst,
            options.rotation.0,
            options.rotation.1,
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
