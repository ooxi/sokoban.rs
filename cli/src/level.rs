use core::position::Position;
use core::tile::Tile;
use core::world::World;

// @see https://www.janko.at/Spiele/Sokoban/GarciaAlberto.htm
pub fn two() -> World {
    let mut world = World::new();

    world.set_tile(Position::new(0, 0), Tile::Floor);
    world.set_tile(Position::new(1, 0), Tile::Floor);
    world.set_tile(Position::new(2, 0), Tile::Floor);
    world.set_tile(Position::new(3, 0), Tile::Floor);
    world.set_tile(Position::new(4, 0), Tile::Floor);

    world.set_tile(Position::new(0, 1), Tile::Crate);
    world.set_tile(Position::new(1, 1), Tile::Crate);
    world.set_tile(Position::new(2, 1), Tile::Floor);
    world.set_tile(Position::new(3, 1), Tile::Crate);
    world.set_tile(Position::new(4, 1), Tile::Floor);

    world.set_tile(Position::new(0, 2), Tile::Floor);
    world.set_tile(Position::new(1, 2), Tile::Crate);
    world.set_tile(Position::new(2, 2), Tile::Floor);
    world.set_tile(Position::new(3, 2), Tile::Wall);
    world.set_tile(Position::new(4, 2), Tile::Floor);

    world.set_tile(Position::new(0, 3), Tile::Floor);
    world.set_tile(Position::new(1, 3), Tile::Wall);
    world.set_tile(Position::new(2, 3), Tile::Floor);
    world.set_tile(Position::new(3, 3), Tile::Wall);
    world.set_tile(Position::new(4, 3), Tile::Floor);

    world.set_tile(Position::new(0, 4), Tile::Floor);
    world.set_tile(Position::new(1, 4), Tile::Wall);
    world.set_tile(Position::new(2, 4), Tile::Player);
    world.set_tile(Position::new(3, 4), Tile::Wall);
    world.set_tile(Position::new(4, 4), Tile::Floor);

    world.set_tile(Position::new(0, 5), Tile::Floor);
    world.set_tile(Position::new(1, 5), Tile::Wall);
    world.set_tile(Position::new(2, 5), Tile::Floor);
    world.set_tile(Position::new(3, 5), Tile::Wall);
    world.set_tile(Position::new(4, 5), Tile::Floor);

    world.set_tile(Position::new(0, 6), Tile::Floor);
    world.set_tile(Position::new(1, 6), Tile::Wall);
    world.set_tile(Position::new(2, 6), Tile::Floor);
    world.set_tile(Position::new(3, 6), Tile::Crate);
    world.set_tile(Position::new(4, 6), Tile::Floor);

    world.set_tile(Position::new(0, 7), Tile::Floor);
    world.set_tile(Position::new(1, 7), Tile::Crate);
    world.set_tile(Position::new(2, 7), Tile::Crate);
    world.set_tile(Position::new(3, 7), Tile::Crate);
    world.set_tile(Position::new(4, 7), Tile::Floor);

    world.set_tile(Position::new(0, 8), Tile::Floor);
    world.set_tile(Position::new(1, 8), Tile::Floor);
    world.set_tile(Position::new(2, 8), Tile::Floor);
    world.set_tile(Position::new(3, 8), Tile::Floor);
    world.set_tile(Position::new(4, 8), Tile::Floor);

    world.add_target(Position::new(0, 4));
    world.add_target(Position::new(0, 5));

    world.add_target(Position::new(2, 3));
    world.add_target(Position::new(2, 4));
    world.add_target(Position::new(2, 5));

    world.add_target(Position::new(4, 2));
    world.add_target(Position::new(4, 3));
    world.add_target(Position::new(4, 4));

    return world;
}
