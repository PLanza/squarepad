use crate::drawable::Drawable;
use crate::menus::BottomMenu;
use crate::pages::Pages;
use crate::renderer::Renderer;
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

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let tex_creator = canvas.texture_creator();

        let event_pump = sdl_context.sdl.event_pump()?;

        Ok(App {
            canvas,
            tex_creator,
            event_pump,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut renderer = Renderer::new(&mut self.canvas, &self.tex_creator);
        renderer.clear();
        renderer.update();

        let pages = Pages::new(
            (42, 59),
            Path::new("assets/white_squared.png"),
            &mut renderer,
        )?;

        let bottom_menu = BottomMenu::new(
            vec![Path::new("assets/page_style_button.png")],
            &mut renderer,
        )?;

        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::MouseWheel { y: scroll, .. } => renderer.scroll(scroll),
                    _ => {}
                }
            }

            renderer.clear();
            pages.draw(&mut renderer)?;
            bottom_menu.draw(&mut renderer)?;
            renderer.update();
        }

        Ok(())
    }
}
