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

pub struct Button<'b> {
    pub id: Uuid,
    rect: Rect,
    state: ButtonState,
    toggled: bool,
    on_click: Box<dyn Fn(&Self) -> Result<(), String>>,
    pub(super) pages: Rc<RefCell<Pages<'b>>>,
}

impl<'b> Button<'b> {
    pub fn new(
        position: (i32, i32),
        image_path: &Path,
        renderer: Rc<RefCell<Renderer>>,
        pages: Rc<RefCell<Pages<'b>>>,
    ) -> Result<Button<'b>, String> {
        let surface = Surface::from_file(image_path)?;
        let id = Uuid::new_v4();

        renderer.borrow_mut().create_texture(id, vec![&surface])?;

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

impl<'f> Drawable for Button<'f> {
    fn draw(&self, renderer: Rc<RefCell<Renderer>>) -> Result<(), String> {
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
                renderer.borrow_mut().draw_fill_rect(
                    self.rect,
                    Color::RGBA(0, 0, 0, 100),
                    false,
                )?;
            }
            ButtonState::CLICKED => {
                renderer
                    .borrow_mut()
                    .draw_fill_rect(self.rect, Color::RGBA(0, 0, 0, 50), false)?;
            }
            _ => (),
        }

        renderer.borrow_mut().draw_texture(self.id, 0, options)?;

        Ok(())
    }
}
