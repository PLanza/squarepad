use crate::position::Position;

use sdl2::ttf::Font;

pub struct TextBox {
    id: uuid::Uuid,
    position: Position,
}
