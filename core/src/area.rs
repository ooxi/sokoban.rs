use crate::position::Position;

pub struct Area {
    upperLeft: Position,
    lowerRight: Position,
}

impl Area {
    pub fn new(upperLeft: Position, lowerRight: Position) -> Area {
        return Area {
            upperLeft: upperLeft,
            lowerRight: lowerRight,
        };
    }

    pub fn upperLeft(&self) -> Position {
        return self.upperLeft;
    }

    pub fn lowerRight(&self) -> Position {
        return self.lowerRight;
    }
}
