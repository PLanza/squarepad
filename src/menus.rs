use crate::drawable::{DrawOptions, Drawable};
use crate::renderer::Renderer;

use std::path::Path;

use uuid::Uuid;

use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

const BOTTOM_MENU_HEIGHT: u32 = 30;

enum ButtonState {
    OFF,
    HOVER,
    CLICK,
}

struct BottomButton {
    pub id: Uuid,
    width: u32,
    state: ButtonState,
}

impl BottomButton {
    pub fn new(image_path: &Path, renderer: &mut Renderer) -> Result<BottomButton, String> {
        let surface = Surface::from_file(image_path)?;
        let width = surface.width();
        let id = Uuid::new_v4();

        renderer.create_texture(id, vec![&surface])?;

        Ok(BottomButton {
            id,
            width,
            state: ButtonState::OFF,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

pub struct BottomMenu {
    pub id: Uuid,
    padding: u32,
    buttons: Vec<BottomButton>,
}

impl BottomMenu {
    pub fn new(image_paths: Vec<&Path>, renderer: &mut Renderer) -> Result<BottomMenu, String> {
        let mut buttons = Vec::new();

        for path in image_paths {
            let button = BottomButton::new(path, renderer)?;
            buttons.push(button);
        }

        let id = Uuid::new_v4();

        Ok(BottomMenu {
            id,
            padding: 20,
            buttons,
        })
    }
}

impl Drawable for BottomMenu {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        let (screen_w, screen_h) = renderer.dimensions();

        let top = (screen_h - BOTTOM_MENU_HEIGHT) as i32;

        renderer.draw_fill_rect(
            Rect::new(0, top, screen_w, BOTTOM_MENU_HEIGHT),
            Color::WHITE,
            false,
        )?;

        let mut start: u32 = 30;
        for button in &self.buttons {
            // Darken area if mouse over or clicked
            match &button.state {
                ButtonState::OFF => (),
                ButtonState::HOVER => renderer.draw_fill_rect(
                    Rect::new(start as i32, top, button.width(), BOTTOM_MENU_HEIGHT),
                    Color::RGBA(0, 0, 0, 100),
                    false,
                )?,
                ButtonState::CLICK => renderer.draw_fill_rect(
                    Rect::new(start as i32, top, button.width(), BOTTOM_MENU_HEIGHT),
                    Color::RGBA(0, 0, 0, 50),
                    false,
                )?,
            }

            let options = DrawOptions {
                src: None,
                dst: Some(Rect::new(
                    start as i32,
                    top,
                    button.width(),
                    BOTTOM_MENU_HEIGHT,
                )),
                rotation: None,
                flip_h: false,
                flip_v: false,
                on_world: false,
            };

            renderer.draw_texture(button.id, 0, options)?;

            start += button.width() + self.padding;
        }

        renderer.draw_line((0, top - 1), (screen_w as i32, top - 1), Color::GRAY)?;

        Ok(())
    }
}
