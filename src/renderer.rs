use crate::drawable::DrawOptions;

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use uuid::Uuid;

pub struct Renderer<'c, 't> {
    canvas: &'c mut WindowCanvas,
    tex_creator: &'t TextureCreator<WindowContext>,
    textures: HashMap<Uuid, Vec<Texture<'t>>>,
    camera: Rect,
}

impl<'c, 't> Renderer<'c, 't> {
    // Only one Renderer will ever be constructed. The default settings set the window to be
    // maximized to the default display, allowing the window to be resized.
    pub(super) fn new(
        canvas: &'c mut WindowCanvas,
        tex_creator: &'t TextureCreator<WindowContext>,
    ) -> Rc<RefCell<Renderer<'c, 't>>> {
        let camera = Rect::new(0, -200, canvas.window().size().0, canvas.window().size().1);

        Rc::new(RefCell::new(Renderer {
            canvas,
            tex_creator,
            textures: HashMap::new(),
            camera,
        }))
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

    fn world_to_screen(&self, rect: Rect) -> Rect {
        Rect::new(
            rect.x - self.camera.x,
            rect.y - self.camera.y,
            rect.width(),
            rect.height(),
        )
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

    pub fn scroll(&mut self, scroll: i32) {
        let new_y = (self.camera.y - scroll * 31).max(-200);

        self.camera = Rect::new(
            self.camera.x,
            new_y,
            self.camera.width(),
            self.camera.height(),
        )
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

        // Convert from world to screen coordinates
        let dst = match options.dst {
            None => None,
            Some(rect) => {
                if options.on_world {
                    Some(self.world_to_screen(rect))
                } else {
                    Some(rect)
                }
            }
        };

        self.canvas.copy_ex(
            &textures[index],
            options.src,
            dst,
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
        start: (i32, i32),
        end: (i32, i32),
        color: Color,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_line(start, end)?;
        Ok(())
    }

    pub fn draw_fill_rect(
        &mut self,
        rect: Rect,
        color: Color,
        on_world: bool,
    ) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        if on_world {
            self.canvas.fill_rect(self.world_to_screen(rect))?;
        } else {
            self.canvas.fill_rect(rect)?;
        }

        Ok(())
    }
}
