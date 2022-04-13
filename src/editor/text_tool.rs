use crate::mark::textbox::TextBox;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, TextInputUtil};

// Contains the state of any text being inputted
pub struct TextTool {
    text_input: TextInputUtil, // May need to share this later with code tool
    input_string: String,
    text_box: Option<Rc<RefCell<TextBox>>>,
}

impl TextTool {
    pub fn new(text_input: TextInputUtil) -> TextTool {
        TextTool {
            text_input,
            input_string: String::new(),
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

    pub fn handle_event(&mut self, event: &Event, renderer: &mut Renderer) {
        match event {
            Event::TextInput { text, .. } => {
                self.input_string.push_str(text);
                println!("{}", self.input_string);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Backspace),
                ..
            } => {
                if !self.input_string.is_empty() {
                    self.input_string.pop();
                    println!("{}", self.input_string);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => self.input_string.push('\n'),
            _ => (),
        }
    }

    pub fn paste(&mut self, text: String) {
        if self.text_input.is_active() {
            self.input_string.push_str(&text);
        }
    }
}
