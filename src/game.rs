use crate::board::*;
use crate::strategies::Player;

/// Game represents a single game played between two players
pub struct Game {
    board: Board,
}

impl Game {
    /// Creates and plays a game between two players, given their strategies.
    pub fn new(x: &impl Player, o: &impl Player) -> Game {
        let mut board = Board::new();

        let mut turn_number = 1;
        while board.get_game_result() == GameResult::InProgress {
            let token = if turn_number % 2 == 1 { Token::X } else { Token::O };
            println!("It is {}'s turn", token);
            let their_move = if turn_number % 2 == 1 { x.take_turn(&board) } else { o.take_turn(&board) };
            board.add_move(token, their_move);
            println!("{}", board);
            turn_number += 1;
        }

        Game { board }
    }

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
        let x = SimpletonPlayer{};
        let o = SimpletonPlayer{};
        let game = Game::new(&x, &o);
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
            let x = RandomPlayer{};
            let o = RandomPlayer{};
            let game = Game::new(&x, &o);
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

