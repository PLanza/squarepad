use crate::Document;
use crate::Drawable;
use crate::Renderer;
use crate::SdlContext;

use std::path::Path;

use sdl2::event::Event;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::video::WindowContext;

pub struct App {
    canvas: WindowCanvas,
    tex_creator: TextureCreator<WindowContext>,
    event_pump: sdl2::EventPump,
}

impl App {
    // Initializes the canvas and the texture creator for the renderer
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

        let event_pump = sdl_context.sdl.event_pump()?;

        Ok(App {
            canvas,
            tex_creator,
            event_pump,
        })
    }

    // TODO: add startup function that creates document and renderer,
    // setting the appropriate world size.

    pub fn run(&mut self) -> Result<(), String> {
        let mut renderer = Renderer::new(&mut self.canvas, &self.tex_creator);
        renderer.clear();
        renderer.update();

        let document = Document::new((42, 59), Path::new("assets/basic_sheet.png"), &mut renderer)?;

        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::MouseWheel { y: scroll, .. } => renderer.scroll(scroll),
                    _ => {}
                }
            }

            renderer.clear();
            document.draw(&mut renderer)?;
            renderer.update();
        }

        Ok(())
    }
}