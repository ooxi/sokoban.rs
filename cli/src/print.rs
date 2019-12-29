use core::position::Position;
use core::tile::Tile;
use core::world::World;

pub trait Print {
    fn print(&self);
}

impl Print for Tile {
    fn print(&self) {
        match self {
            Tile::Crate => print!("$$"),
            Tile::Floor => print!("  "),
            Tile::Player => print!("@@"),
            Tile::Wall => print!("##"),
        }
    }
}

impl Print for World {
    fn print(&self) {
        let area = self.area();

        for y in (area.upper_left().y() - 1)..(area.lower_right().y() + 2) {
            for x in (area.upper_left().x() - 1)..(area.lower_right().x() + 2) {
                let position = Position::new(x, y);
                let tile = self.get_tile(position);

                if (Tile::Floor == tile) && self.is_target(position) {
                    print!("..");
                } else {
                    self.get_tile(position).print();
                }
            }
            println!("");
        }
    }
}
