use crate::mark::textbox::TextBox;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, TextInputUtil};

// Contains the state of any text being inputted
pub struct TextTool {
    text_input: TextInputUtil, // May need to share this later with code tool
    text_box: Option<Rc<RefCell<TextBox>>>,
}

impl TextTool {
    pub fn new(text_input: TextInputUtil) -> TextTool {
        TextTool {
            text_input,
            text_box: None,
        }
    }

    pub fn start_input(&mut self, text_box: Rc<RefCell<TextBox>>) {
        self.text_box = Some(text_box);
        self.text_input.start()
    }
    pub fn stop_input(&mut self) {
        self.text_box = None;
        self.text_input.stop()
    }

    pub fn handle_event(&mut self, event: &Event, renderer: &mut Renderer) -> Result<(), String> {
        match &self.text_box {
            Some(text_box) => match event {
                Event::TextInput { text, .. } => text_box.borrow_mut().push_str(text, renderer),
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => text_box.borrow_mut().pop(renderer),
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => Ok(text_box.borrow_mut().new_line()),
                _ => Ok(()),
            },
            None => Ok(()),
        }
    }

    pub fn paste(&mut self, text: String) {}
}
