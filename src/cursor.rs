use crate::drawable::Drawable;
use crate::editor::Editor;
use crate::position::{PageSquare, Position};
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub struct Cursor {
    position: Position,
    editor: Rc<RefCell<Editor>>,
    current_page_square: Option<PageSquare>, // (page#, x, y) of the square that the cursor is on
    click_page_square: Option<PageSquare>,
}

impl Cursor {
    pub fn new(editor: Rc<RefCell<Editor>>) -> Cursor {
        Cursor {
            position: Position::FreeOnScreen(0, 0),
            editor,
            current_page_square: None,
            click_page_square: None,
        }
    }

    // Only updates the position of the cursor
    pub fn handle_event(&mut self, e: &Event, camera: Rect) -> Result<(), String> {
        match e {
            // Updates cursor position
            Event::MouseMotion { x, y, .. } => {
                self.position = Position::FreeOnScreen(*x, *y);

                let editor = self.editor.borrow();
                let pages = editor.get_pages();
                let square_size = pages.square_size();

                match pages.page_contains(self.position, camera) {
                    None => self.current_page_square = None,
                    Some(i) => {
                        // The FreeOnScreen position of the page that the cursor is on top of
                        let p = pages
                            .get_page_position(i)
                            .to_free_on_screen(None, Some(camera))?;

                        let d = Position::add(self.position, -p.x(), -p.y());

                        self.current_page_square = Some(PageSquare::new(
                            i,
                            (d.x() as u32 / square_size, d.y() as u32 / square_size),
                            self.editor.borrow().get_pages(),
                        )?);
                    }
                }

                Ok(())
            }
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                self.click_page_square = self.current_page_square;
                Ok(())
            }
            Event::MouseButtonUp {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                match self.current_page_square {
                    Some(current) => match self.click_page_square {
                        Some(click) => {
                            if click.eq(&current) {
                                self.editor.borrow_mut().handle_click(click)
                            }
                        }
                        None => (),
                    },
                    None => (),
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn page_square(&self) -> Option<PageSquare> {
        self.current_page_square
    }
}

impl Drawable for Cursor {
    // Draws a box around the square where the cursor finds itself in
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        let editor = self.editor.borrow();
        let pages = editor.get_pages();
        let square_size = pages.square_size();

        // The FreeOnScreen position of the page that the cursor is on top of
        let p;
        match pages.page_contains(self.position, renderer.camera()) {
            None => return Ok(()),
            Some(i) => {
                p = pages
                    .get_page_position(i)
                    .to_free_on_screen(None, Some(renderer.camera()))?
            }
        }

        // Vector math to calculate position of cursor box
        let d = Position::add(self.position, -p.x(), -p.y());
        let s = Position::add(
            p,
            (d.x() / square_size as i32) * square_size as i32,
            (d.y() / square_size as i32) * square_size as i32,
        );

        // Draw the cursor box
        renderer.draw_rect(s, 2, (square_size, square_size), Color::BLACK)?;

        // Draw semi transparent rectangles in the four directions,
        // from the cursor box to the edeges of the page
        renderer.draw_fill_rect(
            Position::FreeOnScreen(s.x(), p.y()),
            (square_size - 1, (s.y() - p.y()) as u32),
            Color::RGBA(0, 0, 0, 50),
        )?;
        renderer.draw_fill_rect(
            Position::FreeOnScreen(p.x(), s.y()),
            ((s.x() - p.x()) as u32, square_size - 1),
            Color::RGBA(0, 0, 0, 50),
        )?;
        renderer.draw_fill_rect(
            Position::FreeOnScreen(s.x(), s.y() + square_size as i32),
            (
                square_size - 1,
                (p.y() + pages.page_height() as i32 - s.y()) as u32 - (square_size - 1),
            ),
            Color::RGBA(0, 0, 0, 50),
        )?;
        renderer.draw_fill_rect(
            Position::FreeOnScreen(s.x() + square_size as i32, s.y()),
            (
                (p.x() + pages.page_width() as i32 - s.x()) as u32 - (square_size - 1),
                square_size - 1,
            ),
            Color::RGBA(0, 0, 0, 50),
        )?;

        Ok(())
    }
}
