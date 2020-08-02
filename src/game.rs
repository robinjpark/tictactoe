use crate::board::*;

#[allow(dead_code)]
/// Game represents a single game played between two players
struct Game {
    board: Board,
}

impl Game {
    #[allow(dead_code)]
    /// Creates and plays a game between two players, given their strategies.
    pub fn new(strategy_x: fn (&Board) -> Position, strategy_o: fn (&Board) -> Position) -> Game {
        let mut board = Board::new();

        let mut turn_number = 1;
        while board.get_game_result() == GameResult::InProgress {
            let player = if turn_number % 2 == 1 { Token::X } else { Token::O };
            let their_strategy = if turn_number % 2 == 1 { strategy_x } else { strategy_o };
            let their_move = their_strategy(&board);
            board.add_move(player, their_move);
            turn_number += 1;
        }

        Game { board }
    }

    #[allow(dead_code)]
    /// Returns the result of the game
    pub fn result(&self) -> GameResult {
        self.board.get_game_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategies::*;

    #[test]
    fn test_game_between_simpletons() {
        let game = Game::new(simpleton_player::take_turn, simpleton_player::take_turn);
        assert_ne!(game.result(), GameResult::InProgress);
        assert_eq!(game.result(), GameResult::Win(Token::X));
    }

    #[test]
    fn test_game_between_random_players() {
        const GAME_COUNT:u32 = 1000; // 100 games is enough to make reasonable predictions about outcomes.

        let mut draw_count = 0;
        let mut x_win_count = 0;
        let mut o_win_count = 0;
        println! ("Simulating {} games between players who play randomly.", GAME_COUNT);
        for _i in 1..GAME_COUNT+1 {
            let game = Game::new(random_player::take_turn, random_player::take_turn);
            match game.result() {
                GameResult::Draw => draw_count += 1,
                GameResult::Win(Token::X) => x_win_count += 1,
                GameResult::Win(Token::O) => o_win_count += 1,
                GameResult::InProgress => panic!("game should not be still in progress!"),
            }
        }
        println! ("Player X won {} games.", x_win_count);
        println! ("Player O won {} games.", o_win_count);
        println! ("{} games ended in a draw.", draw_count);
        println! ("With {} games played, the probability of having each player win some games approaches 100%", GAME_COUNT);
        println! ("Similarly, the probability of having some games end in a draw approaches 100%");
        println! ("Since X goes first, and has the advantage, the probability of X winning more games than O approaches 100%.");
        assert_ne!(x_win_count, 0, "X should have won at least one of the {} games played!", GAME_COUNT);
        assert_ne!(o_win_count, 0, "O should have won at least one of the {} games played!", GAME_COUNT);
        assert_gt!(x_win_count, o_win_count, "X should have won more games than O!");
        assert_ne!(draw_count, 0, "Some games should have ended in a draw!");
        assert_eq!(x_win_count + o_win_count + draw_count, GAME_COUNT);
        //panic!("Uncomment me to check the output");
    }
}

