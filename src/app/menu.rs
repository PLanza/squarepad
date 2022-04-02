use super::button::Button;
use crate::drawable::Drawable;
use crate::renderer::Renderer;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use uuid::Uuid;

pub enum MenuAlignment {
    Vertical = 0,
    Horizontal = 1,
}

pub struct Menu {
    pub id: Uuid,
    rect: Rect,
    alignment: MenuAlignment,
    border_thickenss: u32, // in pixels
    color: Color,
    border_color: Color,
    padding: (i32, i32),
    buttons: Vec<Button>,
}

impl Menu {
    pub fn new(rect: Rect, alignment: MenuAlignment) -> Menu {
        Menu {
            id: Uuid::new_v4(),
            alignment,
            rect,
            border_thickenss: 3,
            color: Color::WHITE,
            border_color: Color::GRAY,
            padding: (30, 0),
            buttons: Vec::new(),
        }
    }

    pub fn set_position(&mut self, position: (i32, i32)) {
        self.rect = Rect::new(
            position.0,
            position.1,
            self.rect.width(),
            self.rect.height(),
        )
    }

    pub fn set_dimensions(&mut self, dimensions: (u32, u32)) {
        self.rect = Rect::new(self.rect.x(), self.rect.y(), dimensions.0, dimensions.1)
    }

    pub fn set_border_thickness(&mut self, border_thickenss: u32) {
        self.border_thickenss = border_thickenss
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

    pub fn add_button(&mut self, mut new_button: Button) {
        let mut position = match self.alignment {
            MenuAlignment::Horizontal => (self.padding.0, self.rect.y + self.padding.1),
            MenuAlignment::Vertical => (self.rect.x + self.padding.0, self.padding.1),
        };

        for button in &mut self.buttons {
            match self.alignment {
                MenuAlignment::Horizontal => {
                    position.0 += button.width() as i32 + self.padding.0;
                }
                MenuAlignment::Vertical => {
                    position.1 += button.height() as i32 + self.padding.1;
                }
            };
        }
        new_button.set_position(position);

        self.buttons.push(new_button)
    }

    pub fn handle_button_events(&mut self, event: &Event) -> Result<(), String> {
        for button in &mut self.buttons {
            button.handle_event(event)?;
        }

        Ok(())
    }
}

impl Drawable for Menu {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        // draw border
        renderer.draw_fill_rect(
            Rect::new(
                self.rect.x - self.border_thickenss as i32,
                self.rect.y - self.border_thickenss as i32,
                self.rect.width() + 2 * self.border_thickenss,
                self.rect.height() + 2 * self.border_thickenss,
            ),
            self.border_color,
            false,
        )?;

        // draw center
        renderer.draw_fill_rect(self.rect, self.color, false)?;

        for button in &self.buttons {
            button.draw(renderer)?;
        }

        Ok(())
    }
}
