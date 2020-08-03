
use crate::board::*;

pub trait Player {
    fn take_turn (&self, board: &Board) -> Position;
}

