use crate::position::Position;

pub struct Area {
    upper_left: Position,
    lower_right: Position,
}

impl Area {
    pub fn new(upper_left: Position, lower_right: Position) -> Area {
        return Area {
            upper_left: upper_left,
            lower_right: lower_right,
        };
    }

    pub fn upper_left(&self) -> Position {
        return self.upper_left;
    }

    pub fn lower_right(&self) -> Position {
        return self.lower_right;
    }
}
