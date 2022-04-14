pub mod button;
pub mod menu;
pub mod pages;
pub mod setup;

use self::button::Button;
use self::menu::Menu;
use self::pages::Pages;
use crate::cursor::Cursor;
use crate::drawable::Drawable;
use crate::editor::Editor;
use crate::position::Position;
use crate::renderer::Renderer;
use crate::SdlContext;

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use sdl2::clipboard::ClipboardUtil;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::TextInputUtil;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

// This struct controls the setup up and running stages of the application
pub struct App<'a> {
    canvas: WindowCanvas,
    tex_creator: TextureCreator<WindowContext>,
    event_pump: sdl2::EventPump,
    text_input: TextInputUtil, // Only used by the text tool
    clipboard: ClipboardUtil,  // Passed on to the editor which handles it
    fonts: HashMap<String, Font<'a, 'a>>,
}

impl<'a> App<'a> {
    // Initializes the application
    pub fn init(sdl_context: &SdlContext) -> Result<App, String> {
        let display_bounds = sdl_context.video_subsystem.display_usable_bounds(0)?;

        // Sets window to be maximized and resizable
        let window = sdl_context
            .video_subsystem
            .window("SquarePad", display_bounds.width(), display_bounds.height())
            .resizable()
            .maximized()
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        // Sets the canvas blend mode so that alpha values are rendered properly
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let tex_creator = canvas.texture_creator();
        let event_pump = sdl_context.sdl.event_pump()?;

        // String has format FontName-Style_Point
        let mut font_map = HashMap::new();
        let points: Vec<u16> = vec![30, 32, 36, 48, 60, 72];

        // Load all the fonts in assets/fonts
        for entry in Path::new("assets/fonts")
            .read_dir()
            .map_err(|e| e.to_string())?
        {
            match entry {
                Ok(folder) => {
                    for entry in folder.path().read_dir().map_err(|e| e.to_string())? {
                        match entry {
                            Ok(font) => {
                                if !font.path().extension().unwrap().eq_ignore_ascii_case("ttf") {
                                    continue;
                                }

                                let mut font_name = font
                                    .path()
                                    .file_stem()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                for point in &points {
                                    let temp = font_name.clone();
                                    font_name.push('_');
                                    font_name.push_str(&point.to_string());
                                    font_map.insert(
                                        font_name,
                                        sdl_context.ttf.load_font(font.path(), *point)?,
                                    );
                                    font_name = temp;
                                }
                            }
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }

        Ok(App {
            canvas,
            tex_creator,
            event_pump,
            fonts: font_map,
            text_input: sdl_context.video_subsystem.text_input(),
            clipboard: sdl_context.video_subsystem.clipboard(),
        })
    }

    // Sets up the renderer and all the application's UI components
    fn setup<'c, 'tc, 'ttf>(
        canvas: &'c mut WindowCanvas,
        tex_creator: &'tc TextureCreator<WindowContext>,
        text_input: TextInputUtil,
        clipboard: ClipboardUtil,
        fonts: HashMap<String, Font<'ttf, 'ttf>>,
    ) -> Result<(Renderer<'c, 'tc, 'ttf>, AppComponents), String> {
        let mut renderer = Renderer::new(canvas, tex_creator, fonts);

        // Pages will be handed off to the editor which will perform all changes to it
        let pages = Pages::new((42, 59), &mut renderer)?;
        let editor = Rc::new(RefCell::new(Editor::new(pages, text_input, clipboard)));

        let mut add_page_button = Button::new(
            Position::AnchoredRightBottom(220, 140),
            Path::new("assets/images/add_page_button.png"),
            &mut renderer,
            Rc::clone(&editor),
        )?;

        add_page_button.set_on_click(Box::new(|button| {
            button.editor.borrow_mut().add_page();

            Ok(())
        }));

        let mut remove_page_button = Button::new(
            Position::AnchoredRightBottom(120, 140),
            Path::new("assets/images/remove_page_button.png"),
            &mut renderer,
            Rc::clone(&editor),
        )?;

        remove_page_button.set_on_click(Box::new(|button| {
            button.editor.borrow_mut().remove_page();

            Ok(())
        }));

        let bottom_menu = crate::app::setup::setup_bottom_menu(&mut renderer, Rc::clone(&editor))?;
        let tool_menu = crate::app::setup::setup_tool_menu(&mut renderer, Rc::clone(&editor))?;

        let cursor = Cursor::new(Rc::clone(&editor));

        Ok((
            renderer,
            AppComponents {
                editor,
                cursor,
                menus: vec![bottom_menu, tool_menu],
                buttons: vec![add_page_button, remove_page_button],
            },
        ))
    }

    pub fn run(mut self) -> Result<(), String> {
        // First sets everything up
        let (mut renderer, mut ac) = App::setup(
            &mut self.canvas,
            &self.tex_creator,
            self.text_input,
            self.clipboard,
            self.fonts,
        )?;

        // The main run loop
        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::Window { win_event: e, .. } => match e {
                        // Adjust camera based on window and the pages' dimensions
                        WindowEvent::Resized(width, height) => renderer.set_camera(Rect::new(
                            (ac.editor.borrow().get_pages().page_width() as i32 - width) / 2,
                            renderer.camera().y(),
                            width as u32,
                            height as u32,
                        )),
                        _ => (),
                    },
                    Event::MouseWheel { y, .. } => renderer.scroll(y),
                    _ => {
                        ac.cursor.handle_event(&event, renderer.camera())?;
                        ac.editor.borrow_mut().handle_event(&event, &mut renderer)?;

                        for menu in &mut ac.menus {
                            menu.handle_button_events(&event, renderer.dimensions())?;
                        }
                        for button in &mut ac.buttons {
                            button.handle_event(&event, renderer.dimensions())?;
                        }
                    }
                }
            }

            renderer.clear();
            ac.editor.borrow().get_pages().draw(&mut renderer)?;
            ac.editor.borrow().draw_marks(&mut renderer)?;
            ac.cursor.draw(&mut renderer)?;

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

            renderer.update();
        }

        Ok(())
    }
}

pub struct AppComponents {
    editor: Rc<RefCell<Editor>>,
    cursor: Cursor,
    menus: Vec<Menu>,
    buttons: Vec<Button>,
}
