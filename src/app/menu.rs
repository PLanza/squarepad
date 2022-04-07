use super::button::Button;
use crate::drawable::Drawable;
use crate::position::Position;
use crate::renderer::Renderer;

use sdl2::event::Event;
use sdl2::pixels::Color;

use uuid::Uuid;

pub enum MenuAlignment {
    Vertical = 0,
    Horizontal = 1,
}

pub struct Menu {
    pub id: Uuid,
    position: Position,
    size: (u32, u32),
    alignment: MenuAlignment,
    border_thickness: u32, // in pixels
    color: Color,
    border_color: Color,
    padding: (i32, i32),
    buttons: Vec<Button>,
}

impl Menu {
    pub fn new(position: Position, size: (u32, u32), alignment: MenuAlignment) -> Menu {
        Menu {
            id: Uuid::new_v4(),
            alignment,
            position,
            size,
            border_thickness: 3,
            color: Color::WHITE,
            border_color: Color::GRAY,
            padding: (0, 0),
            buttons: Vec::new(),
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }
    pub fn padding(&self) -> (i32, i32) {
        self.padding
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position
    }

    pub fn set_size(&mut self, size: (u32, u32)) {
        self.size = size
    }

    pub fn set_border_thickness(&mut self, border_thickness: u32) {
        self.border_thickness = border_thickness
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color
    }

    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color
    }

    pub fn set_padding(&mut self, padding: (i32, i32)) {
        self.padding = padding
    }

    // Takes a button and changes its position to align with the menu
    // The position that the button previously held is lost
    pub fn add_button(&mut self, mut new_button: Button) {
        let mut position = Position::add(self.position, self.padding.0, self.padding.1);

        for button in &mut self.buttons {
            match self.alignment {
                MenuAlignment::Horizontal => {
                    position = Position::add(position, button.width() as i32 + self.padding.0, 0);
                }
                MenuAlignment::Vertical => {
                    position = Position::add(position, 0, button.height() as i32 + self.padding.1);
                }
            };
        }
        new_button.set_position(position);

        self.buttons.push(new_button)
    }

    // Passes on the event to all the buttons it contains
    pub fn handle_button_events(
        &mut self,
        event: &Event,
        screen_dimensions: (u32, u32),
    ) -> Result<(), String> {
        for button in &mut self.buttons {
            button.handle_event(event, screen_dimensions)?;
        }

        Ok(())
    }
}

impl Drawable for Menu {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        // draw border
        renderer.draw_fill_rect(
            Position::add(
                self.position
                    .to_free_on_screen(Some(renderer.dimensions()), Some(renderer.camera()))?,
                -(self.border_thickness as i32),
                -(self.border_thickness as i32),
            ),
            (
                self.size.0 + 2 * self.border_thickness,
                self.size.1 + 2 * self.border_thickness,
            ),
            self.border_color,
        )?;

        // draw center
        renderer.draw_fill_rect(self.position, self.size, self.color)?;

        for button in &self.buttons {
            button.draw(renderer)?;
        }

        Ok(())
    }
}
