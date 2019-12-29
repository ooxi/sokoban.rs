use crate::tile::Tile;

#[derive(Debug, PartialEq)]
pub struct State {
    pub a: Tile,
    pub b: Tile,
    pub c: Tile,
}

impl State {
    pub fn new(a: Tile, b: Tile, c: Tile) -> State {
        return State { a: a, b: b, c: c };
    }

    pub fn transition(state: State) -> Result<State, &'static str> {
        match state {
            // Player cannot push a crate into another crate
            State {
                a: Tile::Player,
                b: Tile::Crate,
                c: Tile::Crate,
            } => Ok(state),

            // Player can push a crate onto an empty floor spot
            State {
                a: Tile::Player,
                b: Tile::Crate,
                c: Tile::Floor,
            } => Ok(State {
                a: Tile::Floor,
                b: Tile::Player,
                c: Tile::Crate,
            }),

            // Player cannot push a crate into a wall
            State {
                a: Tile::Player,
                b: Tile::Crate,
                c: Tile::Wall,
            } => Ok(state),

            // Player can always move into an empty floor spot
            State {
                a: Tile::Player,
                b: Tile::Floor,
                c,
            } => Ok(State {
                a: Tile::Floor,
                b: Tile::Player,
                c: c,
            }),

            // Player can never move into a wall
            State {
                a: Tile::Player,
                b: Tile::Wall,
                c,
            } => Ok(State {
                a: Tile::Player,
                b: Tile::Wall,
                c: c,
            }),

            // All other states must not occur, e.g. multiple players or player
            // not on position {@code State::a}
            _ => Err("Invalid transition"),
        }
    }
}

#[test]
fn it_rejects_multiple_players() {
    let test = State {
        a: Tile::Player,
        b: Tile::Player,
        c: Tile::Player,
    };

    assert!(
        State::transition(test).is_err(),
        "Multiple players should be rejected"
    );
}

#[test]
fn it_moves_a_crate() {
    let test = State {
        a: Tile::Player,
        b: Tile::Crate,
        c: Tile::Floor,
    };

    let expected = State {
        a: Tile::Floor,
        b: Tile::Player,
        c: Tile::Crate,
    };

    match State::transition(test) {
        Ok(actual) => assert_eq!(expected, actual),
        _ => assert!(false, "Player should be able to move crate"),
    }
}
