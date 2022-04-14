use super::Mark;
use crate::drawable::DrawOptions;
use crate::position::{PageSquare, Position};
use crate::renderer::Renderer;

use sdl2::pixels::Color;
use sdl2::ttf::FontStyle;

pub struct TextBox {
    id: uuid::Uuid,
    page_square: PageSquare, // Position on page
    lines: Vec<String>,
    line_sizes: Vec<(u32, u32)>,
    font_name: String,
    font_style: FontStyle,
    point: u16,
    color: Color,
    max_width: u32,
}

impl TextBox {
    pub fn new(
        page_square: PageSquare,
        font_name: String,
        font_style: FontStyle,
        point: u16,
        color: Color,
        max_width: u32,
    ) -> TextBox {
        TextBox {
            id: uuid::Uuid::new_v4(),
            page_square,
            line_sizes: vec![],
            lines: vec![],
            font_name,
            font_style,
            point,
            color,
            max_width,
        }
    }

    pub fn push_str(&mut self, string: &str, renderer: &mut Renderer) -> Result<(), String> {
        if self.lines.is_empty() {
            self.lines.push("".to_string());
            self.line_sizes.push((0,0));
        }

        let last_line = self.lines.len() - 1;
        let mut line = self.lines[last_line].clone();
        line.push_str(string);

        if renderer.text_overflow(
            &line,
            &self.font_name,
            self.font_style,
            self.point,
            self.max_width,
        )? {
            self.lines.push(string.to_string());
            self.line_sizes.push((0, 0));
        } else {
            self.lines[last_line] = line;
        }

        self.update(renderer)
    }

    pub fn pop(&mut self, renderer: &mut Renderer) -> Result<(), String> {
        if self.lines.is_empty() {
            return Ok(());
        }

        let last_line = self.lines.len() - 1;
        self.lines[last_line].pop();

        if self.lines[last_line].is_empty() {
            self.lines.pop();
            self.line_sizes.pop();
        }

        self.update(renderer)
    }

    pub fn new_line(&mut self) {
        self.lines.push("".to_string());
        self.line_sizes.push((0, 0));
    }

    pub fn update(&mut self, renderer: &mut Renderer) -> Result<(), String> {
        if self.lines.is_empty() {
            return Ok(());
        }
        let last_line = self.line_sizes.len() - 1;

        self.line_sizes[last_line] = renderer.create_text(
            self.id,
            Some(last_line),
            &self.lines[last_line],
            &self.font_name,
            self.font_style,
            self.point,
            self.color,
        )?;

        Ok(())
    }
}

impl Mark for TextBox {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        for (i, size) in self.line_sizes.iter().enumerate() {
            if *size == (0, 0) {
                continue;
            }

            let options = DrawOptions {
                src: None,
                position: Position::add(
                    self.page_square.position,
                    0,
                    i as i32 * crate::app::pages::SQUARE_SIZE as i32,
                ),
                size: *size,
                rotation: None,
                flip_h: false,
                flip_v: false,
            };

            renderer.draw_texture(self.id, i, options)?;
        }

        Ok(())
    }
}
