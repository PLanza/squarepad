use super::button::Button;
use super::menu::Menu;
use super::pages::PageStyle;
use crate::editor::{Editor, ToolType};
use crate::position::Position;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

// This file just sets up various UI components so that they don't clutter the App's setup function

pub fn setup_bottom_menu(
    renderer: &mut Renderer,
    editor: Rc<RefCell<Editor>>,
) -> Result<Menu, String> {
    // Bottom menu will include options affecting page style
    let mut bottom_menu = Menu::new(
        Position::AnchoredLeftBottom(0, 30),
        (renderer.dimensions().0, 30),
        crate::app::menu::MenuAlignment::Horizontal,
    );
    bottom_menu.set_border_thickness(1);
    bottom_menu.set_padding((30, 0));

    // Toggles between white and beige backgrounds
    let mut page_style_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/page_style_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;

    page_style_button.set_on_click(Box::new(|button| {
        let mut editor = button.editor.borrow_mut();

        match editor.get_pages().style() {
            PageStyle::WhiteSquared => editor.set_pages_style(PageStyle::BeigeSquared),
            PageStyle::WhitePlain => editor.set_pages_style(PageStyle::BeigePlain),
            PageStyle::BeigeSquared => editor.set_pages_style(PageStyle::WhiteSquared),
            PageStyle::BeigePlain => editor.set_pages_style(PageStyle::WhitePlain),
        }

        Ok(())
    }));
    bottom_menu.add_button(page_style_button);

    // Toggles the grid
    let mut grid_toggle_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/grid_toggle_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;

    grid_toggle_button.set_on_click(Box::new(|button| {
        let mut editor = button.editor.borrow_mut();

        match editor.get_pages().style() {
            PageStyle::WhiteSquared => editor.set_pages_style(PageStyle::WhitePlain),
            PageStyle::WhitePlain => editor.set_pages_style(PageStyle::WhiteSquared),
            PageStyle::BeigeSquared => editor.set_pages_style(PageStyle::BeigePlain),
            PageStyle::BeigePlain => editor.set_pages_style(PageStyle::BeigeSquared),
        }

        Ok(())
    }));
    bottom_menu.add_button(grid_toggle_button);

    Ok(bottom_menu)
}

pub fn setup_tool_menu(
    renderer: &mut Renderer,
    editor: Rc<RefCell<Editor>>,
) -> Result<Menu, String> {
    let mut tool_menu = Menu::new(
        Position::AnchoredRightTop(200, 100),
        (140, 740),
        crate::app::menu::MenuAlignment::Vertical,
    );
    tool_menu.set_border_thickness(3);
    tool_menu.set_padding((20, 20));

    let mut move_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/move_tool_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;
    move_tool_button.set_on_click(Box::new(|button| {
        button.editor.borrow_mut().set_tool(ToolType::Move);
        Ok(())
    }));

    tool_menu.add_button(move_tool_button);

    let mut text_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/text_tool_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;
    text_tool_button.set_on_click(Box::new(|button| {
        button.editor.borrow_mut().set_tool(ToolType::Text);
        Ok(())
    }));
    tool_menu.add_button(text_tool_button);

    let mut line_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/line_tool_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;
    line_tool_button.set_on_click(Box::new(|button| {
        button.editor.borrow_mut().set_tool(ToolType::Line);
        Ok(())
    }));
    tool_menu.add_button(line_tool_button);

    let mut bullet_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/bullet_tool_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;
    bullet_tool_button.set_on_click(Box::new(|button| {
        button.editor.borrow_mut().set_tool(ToolType::Bullet);
        Ok(())
    }));
    tool_menu.add_button(bullet_tool_button);

    let mut math_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/math_tool_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;
    math_tool_button.set_on_click(Box::new(|button| {
        button.editor.borrow_mut().set_tool(ToolType::Math);
        Ok(())
    }));
    tool_menu.add_button(math_tool_button);

    let mut code_tool_button = Button::new(
        Position::FreeOnScreen(0, 0),
        Path::new("assets/images/code_tool_button.png"),
        renderer,
        Rc::clone(&editor),
    )?;
    code_tool_button.set_on_click(Box::new(|button| {
        button.editor.borrow_mut().set_tool(ToolType::Code);
        Ok(())
    }));
    tool_menu.add_button(code_tool_button);

    Ok(tool_menu)
}
