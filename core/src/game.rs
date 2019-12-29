use crate::movement::Movement;
use crate::state::State;
use crate::world::World;

pub struct Game {
    world: World,
    moves: u32,
}

impl Game {
    pub fn new(world: World) -> Game {
        return Game {
            world: world,
            moves: 0,
        };
    }

    pub fn r#move(&mut self, movement: Movement) {
        let player = self.world.get_player();
        let offset = movement.offset();

        let positions = (player, player + offset.0, player + offset.1);

        let before = State::new(
            self.world.get_tile(positions.0),
            self.world.get_tile(positions.1),
            self.world.get_tile(positions.2),
        );

        let after = State::transition(before).unwrap();

        self.world.set_tile(positions.0, after.a);
        self.world.set_tile(positions.1, after.b);
        self.world.set_tile(positions.2, after.c);

        self.moves += 1;
    }

    pub fn world(&self) -> &World {
        return &self.world;
    }

    pub fn moves(&self) -> u32 {
        return self.moves;
    }
}
