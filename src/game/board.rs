use std::collections::HashMap;
use bincode::{serialize, Infinite};
use game::ship::Ship;

pub type PlayerId = u8;
pub type Timestep = u32;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Board {
    pub ships: HashMap<PlayerId, Ship>,
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

    pub fn to_bytes(&self) -> Vec<u8> {
        return serialize(self, Infinite).expect("Error serializing game board");
    }

    pub fn advance(&mut self, ms: u32) {
        self.time += ms;
    }
}


#[cfg(test)]
mod test {
    use bincode::deserialize;
    use super::*;

    #[test]
    fn serialization() {
        let mut board = Board::new();
        board.add_ship(1, Ship::at_origin());
        board.add_ship(2, Ship::at_origin());

        let encoded: Vec<u8> = board.to_bytes();
        assert_eq!(encoded.len(), 134);

        let decoded: Board = deserialize(&encoded[..]).unwrap();
        assert_eq!(board, decoded);
    }

    #[test]
    fn test_advance() {
        let mut board = Board::new();
        assert_eq!(0, board.time);
        board.advance(25);
        assert_eq!(25, board.time);
        board.advance(10);
        assert_eq!(35, board.time);
    }
}
