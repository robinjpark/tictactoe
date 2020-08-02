use crate::board::*;
use crate::strategies::*;

pub struct OptimalPlayer {
}

impl Player for OptimalPlayer {
    fn take_turn(&self, board: &Board) -> Position {
        let who_am_i = board.whose_turn();
        let potential_moves = board.empty_positions();
        let mut with_results = Vec::<(Position, GameResult)>::new();

        for potential_move in &potential_moves {
            let mut next_board = board.clone();
            next_board.add_move(who_am_i, *potential_move);
            with_results.push((*potential_move, next_board.get_game_result()));
        }

        for (potential_move, result) in with_results.iter() {
            //assert_ne!(*result, GameResult::InProgress);
            if *result == GameResult::Win(who_am_i) {
                return *potential_move;
            }
        }

        potential_moves[0]
    }
}

impl OptimalPlayer {
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

