use crate::drawable::DrawOptions;
use crate::position::Position;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use std::collections::HashMap;

use uuid::Uuid;

pub struct Renderer<'c, 't> {
    canvas: &'c mut WindowCanvas,
    tex_creator: &'t TextureCreator<WindowContext>,
    textures: HashMap<Uuid, Vec<Texture<'t>>>,
    camera: Rect,
    scroll_max: i32,
}

impl<'c, 't> Renderer<'c, 't> {
    // Only one Renderer will ever be constructed. The default settings set the window to be
    // maximized to the default display, allowing the window to be resized.
    pub(super) fn new(
        canvas: &'c mut WindowCanvas,
        tex_creator: &'t TextureCreator<WindowContext>,
    ) -> Renderer<'c, 't> {
        let camera = Rect::new(0, 0, canvas.window().size().0, canvas.window().size().1);

        Renderer {
            canvas,
            tex_creator,
            textures: HashMap::new(),
            camera,
            scroll_max: 0,
        }
    }

    pub(crate) fn create_texture(
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

    pub fn scroll(&mut self, scroll: i32) {
        // Keep the scrolling within the pages
        let new_y = ((self.camera.y - scroll * 62).max(0)).min(self.scroll_max);

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

    pub fn draw_line(
        &mut self,
        start: Position,
        end: Position,
        color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        let (start, end): ((i32, i32), (i32, i32)) = (
            start
                .to_free_on_screen(Some(self.dimensions()), Some(self.camera))?
                .into(),
            end.to_free_on_screen(Some(self.dimensions()), Some(self.camera))?
                .into(),
        );

        self.canvas.draw_line(start, end)?;
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
}
