use crate::Renderer;
use crate::SdlContext;

use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct App {
    canvas: WindowCanvas,
    tex_creator: TextureCreator<WindowContext>,
}

impl App {
    pub fn init(sdl_context: &SdlContext) -> Result<App, String> {
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

        Ok(App {
            canvas,
            tex_creator,
        })
    }

    pub fn renderer(&mut self) -> Result<Renderer, String> {
        Ok(Renderer::new(&mut self.canvas, &self.tex_creator))
    }
}
