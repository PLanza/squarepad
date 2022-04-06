use crate::drawable::Drawable;
use crate::editor::Editor;
use crate::position::Position;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::pixels::Color;

pub struct Cursor {
    position: Position,
    editor: Rc<RefCell<Editor>>,
}

impl Cursor {
    pub fn new(editor: Rc<RefCell<Editor>>) -> Cursor {
        Cursor {
            position: Position::FreeOnScreen(500, 500),
            editor,
        }
    }

    pub fn handle_event(&mut self, e: &Event) -> Result<(), String> {
        match e {
            Event::MouseMotion { x, y, .. } => {
                self.position = Position::FreeOnScreen(*x, *y);
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl Drawable for Cursor {
    // Draws a square around the square where the cursor finds itself in
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        let editor = self.editor.borrow();
        let pages = editor.borrow_pages();
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

        let d = Position::add(self.position, -p.x(), -p.y());
        let s = Position::add(
            p,
            (d.x() / (square_size as i32 + 1)) * (square_size as i32 + 1),
            (d.y() / (square_size as i32 + 1)) * (square_size as i32 + 1),
        );

        renderer.draw_rect(s, 2, (square_size, square_size), Color::BLACK)?;

        // Draw semi transparent rectangles in the four directions,
        // from the cursor square to the edeges of the page
        renderer.draw_fill_rect(
            Position::FreeOnScreen(s.x(), p.y()),
            (square_size, (s.y() - p.y()) as u32),
            Color::RGBA(0, 0, 0, 50),
        )?;
        renderer.draw_fill_rect(
            Position::FreeOnScreen(p.x(), s.y()),
            ((s.x() - p.x()) as u32, square_size),
            Color::RGBA(0, 0, 0, 50),
        )?;
        renderer.draw_fill_rect(
            Position::FreeOnScreen(s.x(), s.y() + square_size as i32 + 1),
            (
                square_size,
                (p.y() + pages.page_height() as i32 - s.y()) as u32 - square_size,
            ),
            Color::RGBA(0, 0, 0, 50),
        )?;
        renderer.draw_fill_rect(
            Position::FreeOnScreen(s.x() + square_size as i32 + 1, s.y()),
            (
                (p.x() + pages.page_width() as i32 - s.x()) as u32 - square_size,
                square_size,
            ),
            Color::RGBA(0, 0, 0, 50),
        )?;

        Ok(())
    }
}
