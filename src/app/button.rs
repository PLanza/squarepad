use super::pages::Pages;
use crate::drawable::{DrawOptions, Drawable};
use crate::position::Position;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use uuid::Uuid;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::pixels::Color;
use sdl2::surface::Surface;

#[derive(Debug)]
enum ButtonState {
    OFF,
    HOVER,
    CLICKED,
}

pub struct Button {
    pub id: Uuid,
    position: Position,
    size: (u32, u32),
    state: ButtonState,
    toggled: bool,
    on_click: Box<dyn Fn(&Self) -> Result<(), String>>, // Boxed closure for button functionality
    pub(super) pages: Rc<RefCell<Pages>>, // Needed to change pages from within closure
}

impl Button {
    pub fn new(
        position: Position,
        image_path: &Path,
        renderer: &mut Renderer,
        pages: Rc<RefCell<Pages>>,
    ) -> Result<Button, String> {
        let surface = Surface::from_file(image_path)?;
        let id = Uuid::new_v4();

        renderer.create_texture(id, vec![&surface])?;

        Ok(Button {
            id,
            position,
            size: (surface.width(), surface.height()),
            state: ButtonState::OFF,
            toggled: false,
            on_click: Box::new(|_| Ok(())),
            pages,
        })
    }

    pub fn width(&self) -> u32 {
        self.size.0
    }

    pub fn height(&self) -> u32 {
        self.size.1
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_on_click(&mut self, on_click: Box<dyn Fn(&Self) -> Result<(), String>>) {
        self.on_click = on_click;
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position
    }

    // Assumes the point given is in FreeOnScreen
    pub fn contains_point(
        &self,
        x: i32,
        y: i32,
        screen_dimensions: (u32, u32),
    ) -> Result<bool, String> {
        // Assumes self.position is a Position::FreeOnScreen
        let position = self
            .position
            .to_free_on_screen(Some(screen_dimensions), None)?;
        if x >= position.x() && x < position.x() + self.width() as i32 {
            if y >= position.y() && y < position.y() + self.height() as i32 {
                return Ok(true);
            }
        }
        Ok(false)
    }

    // Handles any mouse event dealing with the button
    // Requires screen_dimensions because mouse position is FreeOnScreen which may need to be
    // converted to button position as AnchoredOnScreen
    pub fn handle_event(&mut self, e: &Event, screen_dimensions: (u32, u32)) -> Result<(), String> {
        match e {
            Event::MouseMotion { x, y, .. } => {
                if self.contains_point(*x, *y, screen_dimensions)?
                    && !matches!(self.state, ButtonState::CLICKED)
                {
                    self.state = ButtonState::HOVER;
                } else if !self.contains_point(*x, *y, screen_dimensions)?
                    && !matches!(self.state, ButtonState::CLICKED)
                {
                    self.state = ButtonState::OFF;
                }
                Ok(())
            }
            Event::MouseButtonDown { x, y, .. } => {
                if self.contains_point(*x, *y, screen_dimensions)? {
                    self.state = ButtonState::CLICKED;
                }
                Ok(())
            }
            Event::MouseButtonUp { x, y, .. } => {
                if self.contains_point(*x, *y, screen_dimensions)?
                    && matches!(self.state, ButtonState::CLICKED)
                {
                    self.state = ButtonState::HOVER;
                    self.toggled = !self.toggled;
                    (self.on_click)(self)
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
            position: self.position,
            size: self.size,
            rotation: None,
            flip_h: self.toggled,
            flip_v: false,
        };

        // Shade over button when hover or clicked
        match self.state {
            ButtonState::HOVER => {
                renderer.draw_fill_rect(self.position, self.size, Color::RGBA(0, 0, 0, 100))?;
            }
            ButtonState::CLICKED => {
                renderer.draw_fill_rect(self.position, self.size, Color::RGBA(0, 0, 0, 50))?;
            }
            _ => (),
        }

        renderer.draw_texture(self.id, 0, options)?;

        Ok(())
    }
}
