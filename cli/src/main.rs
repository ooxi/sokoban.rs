mod level;

use core::game::Game;
use core::movement::Movement;
use core::position::Position;
use core::tile::Tile;
use core::world::World;

trait Print {
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

        for y in (area.upperLeft().y() - 1)..(area.lowerRight().y() + 2) {
            for x in (area.upperLeft().x() - 1)..(area.lowerRight().x() + 2) {
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

fn main() {
    let mut game = Game::new(level::two());

    while !game.world().is_won() {
        game.world().print();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.as_ref() {
            "w\n" => game.r#move(Movement::Up),
            "a\n" => game.r#move(Movement::Left),
            "s\n" => game.r#move(Movement::Down),
            "d\n" => game.r#move(Movement::Right),
            "x\n" => break,
            _ => println!("Unknown input!"),
        }
    }

    if game.world().is_won() {
        game.world().print();
        println!("\nYou won!");
    }
}
