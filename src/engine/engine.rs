use std::time::Instant;
use game::board::Board;

pub struct Round {
    started_at: Instant,
    pub board: Board,
    // TODO: physics here
}

impl Round {
    pub fn new() -> Round {
        Round {
            started_at: Instant::now(),
            board: Board::new(),
        }
    }
}
