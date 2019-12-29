use crate::area::Area;
use crate::position::Position;
use crate::tile::Tile;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct World {
    map: HashMap<Position, Tile>,
    targets: HashSet<Position>,
}

impl World {
    pub fn new() -> World {
        return World {
            map: HashMap::new(),
            targets: HashSet::new(),
        };
    }

    // Returns a map's tile on an infinite grid of wall tiles
    //
    // @param position World position you are interested in
    // @return Tile on {@code position}
    pub fn get_tile(&self, position: Position) -> Tile {
        return *self.map.get(&position).unwrap_or(&Tile::Wall);
    }

    // Changes a position to another kind of tile
    pub fn set_tile(&mut self, position: Position, tile: Tile) {
        if Tile::Wall == tile {
            self.map.remove(&position);
        } else {
            self.map.insert(position, tile);
        }
    }

    // @return Upper left and lower right corner of map
    pub fn area(&self) -> Area {
        let any = self.map.keys().next();

        if any.is_none() {
            return Area::new(Position::new(0, 0), Position::new(0, 0));
        }

        let mut min_x = any.unwrap().x();
        let mut min_y = any.unwrap().y();
        let mut max_x = any.unwrap().x();
        let mut max_y = any.unwrap().y();

        for position in self.map.keys() {
            if position.x() < min_x {
                min_x = position.x();
            }
            if position.y() < min_y {
                min_y = position.y();
            }
            if position.x() > max_x {
                max_x = position.x();
            }
            if position.y() > max_y {
                max_y = position.y();
            }
        }

        return Area::new(Position::new(min_x, min_y), Position::new(max_x, max_y));
    }

    // Add a target on a certain position
    pub fn add_target(&mut self, position: Position) {
        self.targets.insert(position);
    }

    // @return The player's position
    pub fn get_player(&self) -> Position {
        for (position, tile) in self.map.iter() {
            if Tile::Player == *tile {
                return *position;
            }
        }
        panic!("World does not contain player");
    }

    // @return true iff all targets have a crate on them
    pub fn is_won(&self) -> bool {
        for target in self.targets.iter() {
            if Tile::Crate != self.get_tile(*target) {
                return false;
            }
        }

        // No target without a crate on it
        return true;
    }

    // @return true iff a position is a target
    pub fn is_target(&self, position: Position) -> bool {
        return self.targets.contains(&position);
    }
}
