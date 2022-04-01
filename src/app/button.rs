use super::pages::Pages;
use crate::drawable::{DrawOptions, Drawable};
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use uuid::Uuid;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;

#[derive(Debug)]
enum ButtonState {
    OFF,
    HOVER,
    CLICKED,
}

pub struct Button {
    pub id: Uuid,
    rect: Rect,
    state: ButtonState,
    toggled: bool,
    on_click: Box<dyn Fn(&Self) -> Result<(), String>>,
    pub(super) pages: Rc<RefCell<Pages>>,
}

impl Button {
    pub fn new(
        position: (i32, i32),
        image_path: &Path,
        renderer: &mut Renderer,
        pages: Rc<RefCell<Pages>>,
    ) -> Result<Button, String> {
        let surface = Surface::from_file(image_path)?;
        let id = Uuid::new_v4();

        renderer.create_texture(id, vec![&surface])?;

        Ok(Button {
            id,
            rect: Rect::new(position.0, position.1, surface.width(), surface.height()),
            state: ButtonState::OFF,
            toggled: false,
            on_click: Box::new(|_| Ok(())),
            pages,
        })
    }

    pub fn width(&self) -> u32 {
        self.rect.width()
    }

    pub fn height(&self) -> u32 {
        self.rect.height()
    }

    pub fn position(&self) -> (i32, i32) {
        (self.rect.x, self.rect.y)
    }

    pub fn is_toggled(&self) -> bool {
        self.toggled
    }

    pub fn set_on_click(&mut self, on_click: Box<dyn Fn(&Self) -> Result<(), String>>) {
        self.on_click = on_click;
    }

    pub fn set_position(&mut self, position: (i32, i32)) {
        self.rect = Rect::new(position.0, position.1, self.width(), self.height())
    }

    pub fn handle_event(&mut self, e: &Event) -> Result<(), String> {
        match e {
            Event::MouseMotion { x, y, .. } => {
                if self.rect.contains_point((*x, *y)) && !matches!(self.state, ButtonState::CLICKED)
                {
                    self.state = ButtonState::HOVER;
                } else if !self.rect.contains_point((*x, *y))
                    && !matches!(self.state, ButtonState::CLICKED)
                {
                    self.state = ButtonState::OFF;
                }
                Ok(())
            }
            Event::MouseButtonDown { x, y, .. } => {
                if self.rect.contains_point((*x, *y)) {
                    self.state = ButtonState::CLICKED;
                }
                Ok(())
            }
            Event::MouseButtonUp { x, y, .. } => {
                if self.rect.contains_point((*x, *y)) {
                    self.state = ButtonState::HOVER;
                    self.toggled = !self.toggled;
                    ((&*self).on_click)(self)
                } else {
                    self.state = ButtonState::OFF;
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

impl Drawable for Button {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        let options = DrawOptions {
            src: None,
            dst: Some(self.rect),
            rotation: None,
            flip_h: self.toggled,
            flip_v: false,
            on_world: false,
        };

        match self.state {
            ButtonState::HOVER => {
                renderer.draw_fill_rect(self.rect, Color::RGBA(0, 0, 0, 100), false)?;
            }
            ButtonState::CLICKED => {
                renderer.draw_fill_rect(self.rect, Color::RGBA(0, 0, 0, 50), false)?;
            }
            _ => (),
        }

        renderer.draw_texture(self.id, 0, options)?;

        Ok(())
    }
}
