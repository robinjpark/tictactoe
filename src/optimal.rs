use crate::board::*;
use crate::strategies::*;

pub struct OptimalPlayer {
}

impl Player for OptimalPlayer {
    fn take_turn(&self, board: &Board) -> Position {
        self.get_best_move(board)
    }
}

impl OptimalPlayer {
    fn get_best_move(&self, board: &Board) -> Position {
        let who_am_i = board.whose_turn();

        let mut positions_and_results = Vec::<(Position, GameResult)>::new();

        for potential_move in board.empty_positions() {
            let mut next_board = board.clone();
            next_board.add_move(who_am_i, potential_move);
            positions_and_results.push((potential_move, next_board.get_game_result()));
        }

        for (potential_move, result) in positions_and_results.iter() {
            //assert_ne!(*result, GameResult::InProgress);
            if *result == GameResult::Win(who_am_i) {
                return *potential_move;
            }
        }

        positions_and_results[0].0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::*;

    #[test]
    #[ignore] // Ignore until it works!!!
    fn test_always_draws() {
        let x = OptimalPlayer{};
        let o = OptimalPlayer{};
        let game = Game::new(&x, &o);
        assert_eq!(game.result(), GameResult::Draw);
    }

    #[test]
    fn test_single_move_left() {
        let player = OptimalPlayer{};
        let board = Board::from_string("XOX\
                                        OO-\
                                        XXO");
        assert_eq!(player.take_turn(&board), Position::new(1,2));
    }

    #[test]
    fn test_winning_move() {
        let player = OptimalPlayer{};
        let board = Board::from_string("XO-\
                                        OO-\
                                        XX-");
        assert_eq!(player.take_turn(&board), Position::new(2,2));

        let player = OptimalPlayer{};
        let board = Board::from_string("XOX\
                                        OO-\
                                        XX-");
        assert_eq!(player.take_turn(&board), Position::new(1,2));
    }

    #[test]
    #[ignore]
    fn test_prevent_loss() {
        let player = OptimalPlayer{};
        let board = Board::from_string("XOO\
                                        OX-\
                                        XX-");
        assert_eq!(player.take_turn(&board), Position::new(2,2));
    }
}

