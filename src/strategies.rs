/// A simpleton_player is one who always picks the first empty location
/// It isn't useful for much, other than testing the game logic.
pub mod simpleton_player {

use crate::board::{Position, Board, GameResult, Player};

#[allow(dead_code)]
fn take_turn (board: &Board) -> Position
{
    let empty_positions = board.empty_positions();
    empty_positions[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let board = Board::new();
        assert_eq!(take_turn(&board), Position::new(0,0));
    }

    #[test]
    fn test_almost_full_board() {
        let board = Board::from_string("XOX\
                                        O-O\
                                        XOX");
        assert_eq!(take_turn(&board), Position::new(1,1));

        let board = Board::from_string("XOX\
                                        XOO\
                                        O-X");
        assert_eq!(take_turn(&board), Position::new(2,1));
    }

    #[test]
    fn test_half_full_board() {
        let board = Board::from_string("X-X\
                                        O-O\
                                        ---");
        assert_eq!(take_turn(&board), Position::new(0,1));
    }

    #[test]
    fn test_fill_the_board() {
        let mut board = Board::new();
        for n in 1..10 {
            let position = take_turn(&board);
            let player =
                if n % 2 == 1 {
                    Player::X
                } else {
                    Player::O
                };
            board.add_move(player, position);
        }
        assert_eq!(board.get_game_result(), GameResult::Win(Player::X));
    }
}

}

