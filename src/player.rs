//! A trait that defines behaviour for a tic-tac-toe player.
use crate::board::*;

/// A generic player of the game.
pub trait Player {
    /// Implement take_turn() to return the desired Position,
    /// given an in-progress game Board.
    fn take_turn(&self, board: &Board) -> Position;
}
