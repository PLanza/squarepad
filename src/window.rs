use crate::SdlContext;

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct Window {
    canvas: WindowCanvas,
}

impl Window {
    pub fn new(sdl_context: &SdlContext) -> Result<Window, String> {
        let display_bounds = sdl_context.video_subsystem.display_usable_bounds(0)?;
        println!("{:?}", display_bounds);

        let window = sdl_context
            .video_subsystem
            .window("SquarePad", display_bounds.width(), display_bounds.height())
            .maximized()
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Window { canvas })
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.canvas.logical_size()
    }

    pub(crate) fn get_canvas(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }

    // Gets the TextureCreator from the screen's canvas, this should only ever be used once
    pub(crate) fn get_texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }
}
