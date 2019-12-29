use crate::position::Position;

pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Movement {
    pub fn offset(&self) -> (Position, Position) {
        match self {
            Movement::Up => (Position::new(0, -1), Position::new(0, -2)),
            Movement::Down => (Position::new(0, 1), Position::new(0, 2)),
            Movement::Left => (Position::new(-1, 0), Position::new(-2, 0)),
            Movement::Right => (Position::new(1, 0), Position::new(2, 0)),
        }
    }
}
