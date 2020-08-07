//! An optimal player, who never loses!
use crate::board::{Board, GameResult, Position};
use crate::player::Player;
#[cfg(test)]
use crate::strategies::RandomPlayer;

/// The OptimalPlayer never loses a game.
pub struct OptimalPlayer {}

impl Player for OptimalPlayer {
    fn take_turn(&mut self, board: &Board) -> Position {
        self.get_best_move(board)
    }
}

impl OptimalPlayer {
    fn get_best_move(&self, board: &Board) -> Position {
        // To speed things up, first check if the center is still available.
        let center = Position::new(1, 1);
        if board.is_position_unused(center) {
            return center;
        }

        let who_am_i = board.whose_turn().unwrap();

        let mut positions_and_results = Vec::<(Position, GameResult)>::with_capacity(8);

        for potential_move in board.empty_positions() {
            let mut next_board = *board;
            next_board.add_move(who_am_i, potential_move);
            let result = self.get_eventual_game_result(&next_board);
            if result == GameResult::Win(who_am_i) {
                return potential_move;
            }
            positions_and_results.push((potential_move, result));
        }

        for (potential_move, result) in positions_and_results.iter() {
            if *result == GameResult::Draw {
                return *potential_move;
            }
        }

        positions_and_results[0].0
    }

    fn get_eventual_game_result(&self, board: &Board) -> GameResult {
        let result = board.get_game_result();
        if result != GameResult::InProgress {
            result
        } else {
            let mut next_board = *board;
            let best_move = self.get_best_move(&next_board);
            next_board.add_move(next_board.whose_turn().unwrap(), best_move);
            self.get_eventual_game_result(&next_board)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Token;
    use crate::game::Game;

    #[test]
    fn test_draws_against_itself() {
        let x = OptimalPlayer {};
        let o = OptimalPlayer {};
        let game = Game::new(x, o);
        assert_eq!(game.result(), GameResult::Draw);
    }

    #[test]
    fn test_never_loses() {
        const NUM_GAMES: u32 = 50;
        for _i in 0..NUM_GAMES {
            let x = OptimalPlayer {};
            let o = RandomPlayer {};
            let game = Game::new(x, o);
            assert_ne!(game.result(), GameResult::Win(Token::O));

            let x = RandomPlayer {};
            let o = OptimalPlayer {};
            let game = Game::new(x, o);
            assert_ne!(game.result(), GameResult::Win(Token::X));
        }
    }

    #[test]
    fn test_single_move_left() {
        let mut player = OptimalPlayer {};
        let board = Board::from_string(
            "XOX\
             OO-\
             XXO",
        );
        assert_eq!(player.take_turn(&board), Position::new(1, 2));
    }

    #[test]
    fn test_winning_move() {
        let mut player = OptimalPlayer {};

        println!("Scenario #1");
        let board = Board::from_string(
            "XO-\
             OO-\
             XX-",
        );
        assert_eq!(
            player.take_turn(&board),
            Position::new(2, 2),
            "Scenario #1 failed"
        );

        println!("Scenario #1");
        let board = Board::from_string(
            "XOX\
             OO-\
             XX-",
        );
        assert_eq!(
            player.take_turn(&board),
            Position::new(1, 2),
            "Scenario #2 failed"
        );
    }

    #[test]
    fn test_prevent_loss() {
        let mut player = OptimalPlayer {};
        let board = Board::from_string(
            "XOO\
             OX-\
             XX-",
        );
        assert_eq!(player.take_turn(&board), Position::new(2, 2));
    }
}
