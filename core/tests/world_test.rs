use core::position::Position;
use core::tile::Tile;
use core::world::World;

#[test]
fn get_set() {
    let mut world = World::new();

    world.set_tile(Position::new(-3, 3), Tile::Player);
    world.set_tile(Position::new(-5, 5), Tile::Crate);
    world.set_tile(Position::new(-5, 5), Tile::Wall);

    assert_eq!(world.get_tile(Position::new(0, 0)), Tile::Wall);
    assert_eq!(world.get_tile(Position::new(-3, 3)), Tile::Player);
    assert_eq!(world.get_tile(Position::new(-5, 5)), Tile::Wall);
}
