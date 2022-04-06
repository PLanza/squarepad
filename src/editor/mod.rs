pub mod text_tool;

use crate::app::pages::{PageStyle, Pages};

use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum ToolType {
    Move = 0,
    Text = 1,
    Line = 2,
    Bullet = 3,
    Math = 4,
    Code = 5,
}

pub struct Editor {
    pages: Rc<RefCell<Pages>>,
    tool_selected: ToolType,
}

impl Editor {
    pub fn new(pages: Rc<RefCell<Pages>>) -> Editor {
        Editor {
            pages,
            tool_selected: ToolType::Move,
        }
    }

    pub fn borrow_pages(&self) -> Ref<'_, Pages> {
        self.pages.borrow()
    }

    pub fn get_tool(&self) -> ToolType {
        self.tool_selected
    }

    pub fn set_tool(&mut self, tool: ToolType) {
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
