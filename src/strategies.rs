/// A simpleton_player is one who always picks the first empty location
/// It isn't useful for much, other than testing the game logic.
use crate::board::{Position, Board, GameResult, Token};

pub mod simpleton_player {

use super::*;

#[allow(dead_code)]
pub fn take_turn (board: &Board) -> Position
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
        for turn in 1..10 {
            let position = take_turn(&board);
            let player =
                if turn % 2 == 1 {
                    Token::X
                } else {
                    Token::O
                };
            board.add_move(player, position);
        }
        assert_eq!(board.get_game_result(), GameResult::Win(Token::X));
    }
}

} // mod simpleton_player

pub mod random_player {

use rand::Rng;

pub fn take_turn (board: &Board) -> Position
{
    let mut rng = rand::thread_rng();

    let empty_positions = board.empty_positions();
    let count = empty_positions.len();
    let position_to_choose = rng.gen_range(0, count);
    empty_positions[position_to_choose]
}

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_played_boards_differ() {
        //for _n in 1..1000 { // Uncomment to tune MAX_SAME_GAMES
        const MAX_GAMES:u32 = 100;
        const MAX_SAME_GAMES:u32 = 35; // Determined by running this many many times!
        println!("Play {} games with players that move randomly.", MAX_GAMES);
        println!("The games should be different!");
        let mut games = Vec::<Board>::new();
        for game in 1..MAX_GAMES+1 {
            let mut board = Board::new();
            for turn in 1..10 {
                let position = take_turn(&board);
                let player =
                    if turn % 2 == 1 {
                        Token::X
                    } else {
                        Token::O
                    };
                board.add_move(player, position);
                if let GameResult::Win(_winner) = board.get_game_result() {
                    break;
                }
            }
            println!("Game #{}:\n{}", game, board);
            games.push(board);
        }

        let mut same_game_count = 0;
        for game1 in 0..MAX_GAMES {
            for game2 in game1+1..MAX_GAMES {
                if games[game1 as usize] == games[game2 as usize] {
                    println!("Games {} and {} are the same!", game1+1, game2+1);
                    same_game_count += 1;
                }
            }
        }

        // Testing indicates that although some games will be the same, not too many will be.
        assert_lt!(same_game_count, MAX_SAME_GAMES+1);
        //} // Uncomment to tune MAX_SAME_GAMES
    }
} // tests

} // random_player

