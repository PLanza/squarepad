pub mod button;
pub mod pages;

use self::button::Button;
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

    fn setup<'r>(renderer: Rc<RefCell<Renderer<'r, 'r>>>) -> Result<AppComponents<'r>, String> {
        let pages = Rc::new(RefCell::new(Pages::new(
            (42, 59),
            Path::new("assets/white_squared.png"),
            Rc::clone(&renderer),
        )?));

        let mut buttons = Vec::new();

        let button_y = renderer.borrow().dimensions().1 as i32 - 30;
        let mut page_style_button = Button::new(
            (30, button_y),
            Path::new("assets/page_style_button.png"),
            renderer,
            Rc::clone(&pages),
        )?;

        page_style_button.set_on_click(Box::new(|button: &Button| {
            if button.is_toggled() {
                button
                    .pages
                    .borrow()
                    .change_page_style(Path::new("assets/beige_squared.png"))
            } else {
                button
                    .pages
                    .borrow()
                    .change_page_style(Path::new("assets/white_squared.png"))
            }
        }));

        buttons.push(page_style_button);

        Ok(AppComponents { pages, buttons })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let renderer = Renderer::new(&mut self.canvas, &self.tex_creator);
        renderer.borrow_mut().clear();
        renderer.borrow_mut().update();

        let mut ac = App::setup(Rc::clone(&renderer))?;

        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::MouseWheel { y: scroll, .. } => renderer.borrow_mut().scroll(scroll),
                    _ => {}
                }

                for button in &mut ac.buttons {
                    button.handle_event(&event)?;
                }
            }

            renderer.borrow_mut().clear();
            ac.pages.borrow_mut().draw(Rc::clone(&renderer))?;
            for button in &ac.buttons {
                button.draw(Rc::clone(&renderer))?;
            }

            renderer.borrow_mut().update();
        }

        Ok(())
    }
}

pub struct AppComponents<'p> {
    pages: Rc<RefCell<Pages<'p>>>,
    buttons: Vec<Button<'p>>,
}
