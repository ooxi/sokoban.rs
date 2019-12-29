mod level;
mod print;

use crate::print::Print;
use core::game::Game;
use core::movement::Movement;
use getch::Getch;

fn main() {
    let getch = Getch::new();
    let mut game = Game::new(level::two());

    while !game.world().is_won() {
        game.world().print();
        println!("");

        const UP_KEY: u8 = 'w' as u8;
        const DOWN_KEY: u8 = 's' as u8;
        const LEFT_KEY: u8 = 'a' as u8;
        const RIGHT_KEY: u8 = 'd' as u8;
        const EXIT_KEY: u8 = 'x' as u8;

        match getch.getch() {
            Ok(UP_KEY) => game.r#move(Movement::Up),
            Ok(DOWN_KEY) => game.r#move(Movement::Down),
            Ok(LEFT_KEY) => game.r#move(Movement::Left),
            Ok(RIGHT_KEY) => game.r#move(Movement::Right),
            Ok(EXIT_KEY) => break,

            Ok(_) => println!(
                "Unknown input!\n\
                 \t{} - Up\n\
                 \t{} - Left\n\
                 \t{} - Down\n\
                 \t{} - Right\n\
                 \t{} - Exit\n\
                 ",
                UP_KEY as char,
                LEFT_KEY as char,
                DOWN_KEY as char,
                RIGHT_KEY as char,
                EXIT_KEY as char,
            ),

            Err(_) => panic!("Cannot read input!"),
        }
    }

    if game.world().is_won() {
        game.world().print();
        println!("\nYou won!");
    }
}
