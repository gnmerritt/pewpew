use std::collections::HashMap;
use game::ship::Ship;

pub type PlayerId = u8;
pub type Timestep = u32;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Board {
    ships: HashMap<PlayerId, Ship>,
    time: Timestep,
}

impl Board {
    pub fn new() -> Board {
        Board {
            ships: HashMap::new(),
            time: 0,
        }
    }

    pub fn add_ship(&mut self, player: PlayerId, ship: Ship) {
        self.ships.insert(player, ship);
    }
}


#[cfg(test)]
mod test {
    use bincode::{serialize, deserialize, SizeLimit};
    use super::*;

    #[test]
    fn serialization() {
        let mut board = Board::new();
        board.add_ship(1, Ship::at_origin());
        board.add_ship(2, Ship::at_origin());

        let encoded: Vec<u8> = serialize(&board, SizeLimit::Infinite).unwrap();
        assert_eq!(encoded.len(), 134);

        let decoded: Board = deserialize(&encoded[..]).unwrap();
        assert_eq!(board, decoded);
    }
}
