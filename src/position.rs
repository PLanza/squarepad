use crate::app::pages::Pages;

use sdl2::rect::Point;
use sdl2::rect::Rect;

// Allows for different coordinate systems to be used interchangably
#[derive(Clone, Copy, Debug, Hash)]
pub enum Position {
    AnchoredLeftBottom(i32, i32), // Offset from bottom and left edges of screen
    AnchoredRightTop(i32, i32),   // Offset from top and right edges of screen
    AnchoredRightBottom(i32, i32), // Offset from borrom and right edge of screen
    FreeOnScreen(i32, i32),       // Same as AnchoredTopLeft, serves as default
    FreeOnWorld(i32, i32),
}

impl Position {
    // Converts from other coordinate systems to FreeOnScreen
    // This is the defualt system as it is the same one that canvas uses
    pub fn to_free_on_screen(
        self,
        screen_dimensions: Option<(u32, u32)>,
        camera: Option<Rect>,
    ) -> Result<Position, String> {
        match self {
            Position::AnchoredLeftBottom(dx, dy) => match screen_dimensions {
                Some((_, s_h)) => Ok(Position::FreeOnScreen(dx, s_h as i32 - dy)),
                None => Err("Cannot convert anchored on-screen position to free on-screen position, without screen dimensions.".to_string())
            },
            Position::AnchoredRightTop(dx, dy) => match screen_dimensions {
                Some((s_w, _)) => Ok(Position::FreeOnScreen(s_w as i32 - dx, dy)),
                None => Err("Cannot convert anchored on-screen position to free on-screen position, without screen dimensions.".to_string())
            },
            Position::AnchoredRightBottom(dx, dy) => match screen_dimensions {
                Some((s_w, s_h)) => Ok(Position::FreeOnScreen(s_w as i32 - dx, s_h as i32 - dy)),
                None => Err("Cannot convert anchored on-screen position to free on-screen position, without screen dimensions.".to_string())
            },
            Position::FreeOnWorld(x, y) => match camera {
                Some(rect) => Ok(Position::FreeOnScreen(x - rect.x(), y - rect.y())),
                None => Err("Cannot convert free on-world position to free on-screen position, without camera.".to_string())
            }
            position => Ok(position)
        }
    }

    pub fn x(self) -> i32 {
        match self {
            Position::AnchoredLeftBottom(x, _) => x,
            Position::AnchoredRightTop(x, _) => x,
            Position::AnchoredRightBottom(x, _) => x,
            Position::FreeOnWorld(x, _) => x,
            Position::FreeOnScreen(x, _) => x,
        }
    }

    pub fn y(self) -> i32 {
        match self {
            Position::AnchoredLeftBottom(_, y) => y,
            Position::AnchoredRightTop(_, y) => y,
            Position::AnchoredRightBottom(_, y) => y,
            Position::FreeOnWorld(_, y) => y,
            Position::FreeOnScreen(_, y) => y,
        }
    }

    // Adds (dx, dy) to any position from the origin at the top-left corner
    // If dx and dy are both positive, this will result in a translation down and to the right
    pub fn add(p: Position, dx: i32, dy: i32) -> Position {
        match p {
            Position::AnchoredLeftBottom(x, y) => Position::AnchoredLeftBottom(x + dx, y - dy),
            Position::AnchoredRightTop(x, y) => Position::AnchoredRightTop(x - dx, y + dy),
            Position::AnchoredRightBottom(x, y) => Position::AnchoredRightBottom(x - dx, y - dy),
            Position::FreeOnWorld(x, y) => Position::FreeOnWorld(x + dx, y + dy),
            Position::FreeOnScreen(x, y) => Position::FreeOnScreen(x + dx, y + dy),
        }
    }
}

impl Into<(i32, i32)> for Position {
    fn into(self) -> (i32, i32) {
        (self.x(), self.y())
    }
}

impl Into<Point> for Position {
    fn into(self) -> Point {
        Point::new(self.x(), self.y())
    }
}

#[derive(Clone, Copy, Debug, Hash)]
pub struct PageSquare {
    pub page: u32,
    pub square: (u32, u32),
    pub position: Position, // The FreeOnWorld position
}

// Represents the position of a square on a particular page
impl PageSquare {
    pub fn new(page: u32, square: (u32, u32), pages: &Pages) -> Result<PageSquare, String> {
        let p = pages.get_page_position(page);
        if square.0 >= pages.page_squares().0 || square.1 >= pages.page_squares().1 {
            return Err("PageSquare out of bounds.".to_string());
        }

        let position = Position::add(
            p,
            (square.0 * pages.square_size()) as i32,
            (square.1 * pages.square_size()) as i32,
        );
        Ok(PageSquare {
            page,
            square,
            position,
        })
    }
}

impl PartialEq for PageSquare {
    fn eq(&self, other: &PageSquare) -> bool {
        self.page == other.page && self.square.eq(&other.square)
    }
}

impl Eq for PageSquare {}
