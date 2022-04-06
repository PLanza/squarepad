pub mod button;
pub mod menu;
pub mod pages;
pub mod setup;

use self::button::Button;
use self::menu::Menu;
use self::pages::PageStyle;
use self::pages::Pages;
use crate::cursor::Cursor;
use crate::drawable::Drawable;
use crate::editor::Editor;
use crate::position::Position;
use crate::renderer::Renderer;
use crate::SdlContext;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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

        // Pages needs to be shared between the components to interact with it
        let pages = Rc::new(RefCell::new(Pages::new((42, 59), &mut renderer)?));
        let editor = Rc::new(RefCell::new(Editor::new(Rc::clone(&pages))));

        let mut bottom_menu = Menu::new(
            Position::AnchoredLeftBottom(0, 30),
            (renderer.dimensions().0, 30),
            crate::app::menu::MenuAlignment::Horizontal,
        );
        bottom_menu.set_border_thickness(1);
        bottom_menu.set_padding((30, 0));

        let mut page_style_button = Button::new(
            Position::FreeOnScreen(0, 0),
            Path::new("assets/page_style_button.png"),
            &mut renderer,
            Rc::clone(&editor),
        )?;

        page_style_button.set_on_click(Box::new(|button| {
            let editor = button.editor.borrow();

            match editor.get_pages_style() {
                PageStyle::WhiteSquared => editor.set_pages_style(PageStyle::BeigeSquared),
                PageStyle::WhitePlain => editor.set_pages_style(PageStyle::BeigePlain),
                PageStyle::BeigeSquared => editor.set_pages_style(PageStyle::WhiteSquared),
                PageStyle::BeigePlain => editor.set_pages_style(PageStyle::WhitePlain),
            }

            Ok(())
        }));

        let mut grid_toggle_button = Button::new(
            Position::FreeOnScreen(0, 0),
            Path::new("assets/grid_toggle_button.png"),
            &mut renderer,
            Rc::clone(&editor),
        )?;

        grid_toggle_button.set_on_click(Box::new(|button| {
            let editor = button.editor.borrow();

            match editor.get_pages_style() {
                PageStyle::WhiteSquared => editor.set_pages_style(PageStyle::WhitePlain),
                PageStyle::WhitePlain => editor.set_pages_style(PageStyle::WhiteSquared),
                PageStyle::BeigeSquared => editor.set_pages_style(PageStyle::BeigePlain),
                PageStyle::BeigePlain => editor.set_pages_style(PageStyle::BeigeSquared),
            }

            Ok(())
        }));
        bottom_menu.add_button(page_style_button);
        bottom_menu.add_button(grid_toggle_button);

        let mut add_page_button = Button::new(
            Position::AnchoredRightBottom(220, 140),
            Path::new("assets/add_page_button.png"),
            &mut renderer,
            Rc::clone(&editor),
        )?;

        add_page_button.set_on_click(Box::new(|button| {
            button.editor.borrow().add_page();

            Ok(())
        }));

        let mut remove_page_button = Button::new(
            Position::AnchoredRightBottom(120, 140),
            Path::new("assets/remove_page_button.png"),
            &mut renderer,
            Rc::clone(&editor),
        )?;

        remove_page_button.set_on_click(Box::new(|button| {
            button.editor.borrow().remove_page();

            Ok(())
        }));

        let tool_menu = crate::app::setup::setup_tool_menu(&mut renderer, Rc::clone(&editor))?;

        let cursor = Cursor::new(Rc::clone(&editor));

        Ok((
            renderer,
            AppComponents {
                pages,
                editor,
                cursor,
                menus: vec![bottom_menu, tool_menu],
                buttons: vec![add_page_button, remove_page_button],
            },
        ))
    }

    pub fn run(&mut self) -> Result<(), String> {
        let (mut renderer, mut ac) = App::setup(&mut self.canvas, &self.tex_creator)?;

        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::Window { win_event: e, .. } => match e {
                        WindowEvent::Resized(width, height) => {
                            // Adjust pages position based on window dimensions
                            renderer.set_camera(Rect::new(
                                (ac.pages.borrow().page_width() as i32 - width) / 2,
                                renderer.camera().y(),
                                width as u32,
                                height as u32,
                            ))
                        }
                        _ => (),
                    },
                    Event::MouseWheel { y: scroll, .. } => renderer.scroll(scroll),
                    _ => (),
                }
                ac.cursor.handle_event(&event)?;

                for menu in &mut ac.menus {
                    menu.handle_button_events(&event, renderer.dimensions())?;
                }
                for button in &mut ac.buttons {
                    button.handle_event(&event, renderer.dimensions())?;
                }
            }

            renderer.clear();
            ac.pages.borrow_mut().draw(&mut renderer)?;
            for menu in &ac.menus {
                menu.draw(&mut renderer)?;
            }
            for button in &ac.buttons {
                button.draw(&mut renderer)?;
            }

            // Draws a rectangle around the currently selected tool
            let tool = ac.editor.borrow().get_tool() as i32;
            let tool_menu = &ac.menus[1];
            renderer.draw_rect(
                Position::add(
                    tool_menu.position(),
                    tool_menu.padding().0 - 1,
                    tool_menu.padding().1 * (tool + 1) + 100 * tool - 1,
                ),
                3,
                (102, 102),
                Color::BLACK,
            )?;

            ac.cursor.draw(&mut renderer)?;

            renderer.update();
        }

        Ok(())
    }
}

pub struct AppComponents {
    pages: Rc<RefCell<Pages>>,
    editor: Rc<RefCell<Editor>>,
    cursor: Cursor,
    menus: Vec<Menu>,
    buttons: Vec<Button>,
}
