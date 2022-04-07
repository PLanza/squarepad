pub mod text_tool;

use self::text_tool::TextTool;
use crate::app::pages::{PageStyle, Pages};

use sdl2::clipboard::ClipboardUtil;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Mod, TextInputUtil};

#[derive(Clone, Copy)]
pub enum ToolType {
    Move = 0,
    Text = 1,
    Line = 2,
    Bullet = 3,
    Math = 4,
    Code = 5,
}

// Handles all changes made to the document
// This means that it also acts as a wrapper for Pages
// This will prevent two changes from happening concurrently
pub struct Editor {
    pages: Pages,
    clipboard: ClipboardUtil,
    tool_selected: ToolType,
    text_tool: TextTool,
}

impl Editor {
    pub fn new(pages: Pages, text_input: TextInputUtil, clipboard: ClipboardUtil) -> Editor {
        Editor {
            pages,
            tool_selected: ToolType::Move,
            text_tool: TextTool::new(text_input),
            clipboard,
        }
    }

    // Only allows immutable behavior to be done on pages
    // All mutable behavior is done through wrapper functions
    pub fn get_pages(&self) -> &Pages {
        &self.pages
    }

    pub fn get_tool(&self) -> ToolType {
        self.tool_selected
    }

    pub fn set_tool(&mut self, tool: ToolType) {
        if self.tool_selected as isize == 1 {
            // Temporary
            self.text_tool.stop_input();
        }
        self.tool_selected = tool
    }

    pub fn set_pages_style(&mut self, style: PageStyle) {
        self.pages.set_style(style)
    }

    pub fn add_page(&mut self) {
        self.pages.add_page()
    }

    pub fn remove_page(&mut self) {
        self.pages.remove_page()
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<(), String> {
        match event {
            Event::MouseButtonUp { .. } => {
                if self.tool_selected as usize == 1 {
                    self.text_tool.start_input();
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::V),
                keymod,
                ..
            } => match *keymod & (Mod::LCTRLMOD | Mod::RCTRLMOD) {
                // If holding down either control
                Mod::LCTRLMOD | Mod::RCTRLMOD => self.paste()?,
                _ => (),
            },
            _ => (),
        }
        self.text_tool.handle_event(event);

        Ok(())
    }

    pub fn paste(&mut self) -> Result<(), String> {
        match self.tool_selected {
            ToolType::Text => self.text_tool.paste(self.clipboard.clipboard_text()?),
            _ => (),
        }
        Ok(())
    }
}
