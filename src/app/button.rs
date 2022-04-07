use crate::drawable::{DrawOptions, Drawable};
use crate::editor::Editor;
use crate::position::Position;
use crate::renderer::Renderer;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use uuid::Uuid;

use sdl2::event::Event;
use sdl2::image::LoadSurface;
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
    position: Position,
    size: (u32, u32),
    state: ButtonState,
    on_click: Box<dyn Fn(&Self) -> Result<(), String>>, // Boxed closure for button functionality
    pub(super) editor: Rc<RefCell<Editor>>, // Needed to change pages from within closure
}

impl Button {
    pub fn new(
        position: Position,
        image_path: &Path,
        renderer: &mut Renderer,
        editor: Rc<RefCell<Editor>>,
    ) -> Result<Button, String> {
        // Button images are split horizontally into three equal parts
        let src = Surface::from_file(image_path)?;
        let (sfc_w, sfc_h) = (src.width() / 3, src.height());

        let mut surface_off = Surface::new(sfc_w, sfc_h, src.pixel_format_enum())?;
        src.blit(Rect::new(0, 0, sfc_w, sfc_h), &mut surface_off, None)?;

        let mut surface_hover = Surface::new(sfc_w, sfc_h, src.pixel_format_enum())?;
        src.blit(
            Rect::new(sfc_w as i32, 0, sfc_w, sfc_h),
            &mut surface_hover,
            None,
        )?;

        let mut surface_click = Surface::new(sfc_w, sfc_h, src.pixel_format_enum())?;
        src.blit(
            Rect::new(sfc_w as i32 * 2, 0, sfc_w, sfc_h),
            &mut surface_click,
            None,
        )?;

        let id = Uuid::new_v4();

        // Each button has 3 associated textures that will be displayed depending on their state
        renderer.create_textures(id, vec![&surface_off, &surface_hover, &surface_click])?;

        Ok(Button {
            id,
            position,
            size: (sfc_w, sfc_h),
            state: ButtonState::OFF,
            on_click: Box::new(|_| Ok(())),
            editor,
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
        // Controls button "state machine"
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
            flip_h: false,
            flip_v: false,
        };

        // Draws texture depending on ButtonState
        match self.state {
            ButtonState::OFF => renderer.draw_texture(self.id, 0, options),
            ButtonState::HOVER => renderer.draw_texture(self.id, 1, options),
            ButtonState::CLICKED => renderer.draw_texture(self.id, 2, options),
        }
    }
}
