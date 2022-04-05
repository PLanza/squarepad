use super::button::Button;
use super::menu::Menu;
use super::pages::Pages;
use crate::position::Position;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub fn setup_tool_menu(renderer: &mut Renderer, pages: Rc<RefCell<Pages>>) -> Result<Menu, String> {
    let mut tool_menu = Menu::new(
        Position::AnchoredRightTop(200, 100),
        (140, 740),
        crate::app::menu::MenuAlignment::Vertical,
    );
    tool_menu.set_border_thickness(3);
    tool_menu.set_padding((20, 20));

    let mut text_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/text_tool_button.png"),
        renderer,
        Rc::clone(&pages),
    )?;
    tool_menu.add_button(text_tool_button);

    let mut move_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/move_tool_button.png"),
        renderer,
        Rc::clone(&pages),
    )?;
    tool_menu.add_button(move_tool_button);

    let mut line_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/line_tool_button.png"),
        renderer,
        Rc::clone(&pages),
    )?;
    tool_menu.add_button(line_tool_button);

    let mut bullet_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/bullet_tool_button.png"),
        renderer,
        Rc::clone(&pages),
    )?;
    tool_menu.add_button(bullet_tool_button);

    let mut math_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/math_tool_button.png"),
        renderer,
        Rc::clone(&pages),
    )?;
    tool_menu.add_button(math_tool_button);

    let mut code_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/code_tool_button.png"),
        renderer,
        Rc::clone(&pages),
    )?;
    tool_menu.add_button(code_tool_button);

    Ok(tool_menu)
}
