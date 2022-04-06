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
    on_page: Option<u32>,
}

impl Cursor {
    pub fn new(editor: Rc<RefCell<Editor>>) -> Cursor {
        Cursor {
            position: Position::FreeOnScreen(500, 500),
            editor,
            on_page: Some(0),
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
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        match self.on_page {
            None => return Ok(()),
            _ => (),
        }

        let editor = self.editor.borrow();
        let pages = editor.borrow_pages();
        let p = pages
            .position()
            .to_free_on_screen(None, Some(renderer.camera()))?;
        let square_size = pages.square_size();

        let d = Position::add(self.position, -p.x(), -p.y());
        let s = Position::add(
            p,
            (d.x() / (square_size as i32 + 1)) * (square_size as i32 + 1),
            (d.y() / (square_size as i32 + 1)) * (square_size as i32 + 1),
        );

        renderer.draw_rect(s, 2, (square_size, square_size), Color::BLACK)?;

        Ok(())
    }
}
