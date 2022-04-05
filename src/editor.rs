use crate::app::pages::{PageStyle, Pages};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Tool {
    Move = 0,
    Text = 1,
    Line = 2,
    Bullet = 3,
    Math = 4,
    Code = 5,
}

pub struct Editor {
    pages: Rc<RefCell<Pages>>,
    tool_selected: Tool,
}

impl Editor {
    pub fn new(pages: Rc<RefCell<Pages>>) -> Editor {
        Editor {
            pages,
            tool_selected: Tool::Move,
        }
    }
    pub fn get_tool(&self) -> Tool {
        self.tool_selected
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.tool_selected = tool
    }

    pub fn get_pages_style(&self) -> PageStyle {
        self.pages.borrow().style()
    }

    pub fn set_pages_style(&self, style: PageStyle) {
        self.pages.borrow_mut().set_style(style)
    }

    pub fn add_page(&self) {
        self.pages.borrow_mut().add_page()
    }

    pub fn remove_page(&self) {
        self.pages.borrow_mut().remove_page()
    }
}
