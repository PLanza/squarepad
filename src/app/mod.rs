pub mod button;
pub mod menu;
pub mod pages;

use self::button::Button;
use self::pages::PageStyle;
use self::pages::Pages;
use crate::drawable::Drawable;
use crate::renderer::Renderer;
use crate::SdlContext;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

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
            .resizable()
            .maximized()
            .position_centered()
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

    // Sets up the renderer and all the application's UI components
    fn setup<'c, 't>(
        canvas: &'c mut WindowCanvas,
        tex_creator: &'t TextureCreator<WindowContext>,
    ) -> Result<(Renderer<'c, 't>, AppComponents), String> {
        let mut renderer = Renderer::new(canvas, tex_creator);

        let pages = Rc::new(RefCell::new(Pages::new((42, 59), &mut renderer)?));

        let mut buttons = Vec::new();

        let button_y = renderer.dimensions().1 as i32 - 90;
        let mut page_style_button = Button::new(
            (30, button_y),
            Path::new("assets/page_style_button.png"),
            &mut renderer,
            Rc::clone(&pages),
        )?;

        page_style_button.set_on_click(Box::new(|button: &Button| {
            let mut pages = button.pages.borrow_mut();

            match pages.style() {
                PageStyle::WhiteSquared => pages.set_style(PageStyle::BeigeSquared),
                PageStyle::WhitePlain => pages.set_style(PageStyle::BeigePlain),
                PageStyle::BeigeSquared => pages.set_style(PageStyle::WhiteSquared),
                PageStyle::BeigePlain => pages.set_style(PageStyle::WhitePlain),
            }

            Ok(())
        }));
        buttons.push(page_style_button);

        let mut grid_toggle_button = Button::new(
            (90, button_y),
            Path::new("assets/grid_toggle_button.png"),
            &mut renderer,
            Rc::clone(&pages),
        )?;

        grid_toggle_button.set_on_click(Box::new(|button: &Button| {
            let mut pages = button.pages.borrow_mut();

            match pages.style() {
                PageStyle::WhiteSquared => pages.set_style(PageStyle::WhitePlain),
                PageStyle::WhitePlain => pages.set_style(PageStyle::WhiteSquared),
                PageStyle::BeigeSquared => pages.set_style(PageStyle::BeigePlain),
                PageStyle::BeigePlain => pages.set_style(PageStyle::BeigeSquared),
            }

            Ok(())
        }));
        buttons.push(grid_toggle_button);

        Ok((renderer, AppComponents { pages, buttons }))
    }

    pub fn run(&mut self) -> Result<(), String> {
        let (mut renderer, mut ac) = App::setup(&mut self.canvas, &self.tex_creator)?;

        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::MouseWheel { y: scroll, .. } => renderer.scroll(scroll),
                    _ => {}
                }

                for button in &mut ac.buttons {
                    button.handle_event(&event)?;
                }
            }

            renderer.clear();
            ac.pages.borrow_mut().draw(&mut renderer)?;
            for button in &ac.buttons {
                button.draw(&mut renderer)?;
            }

            renderer.update();
        }

        Ok(())
    }
}

pub struct AppComponents {
    pages: Rc<RefCell<Pages>>,
    buttons: Vec<Button>,
}
