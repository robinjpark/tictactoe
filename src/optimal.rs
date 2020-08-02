use crate::board::*;
use crate::strategies::*;

pub struct OptimalPlayer {
}

impl Player for OptimalPlayer {
    fn take_turn(&mut self, board: &Board) -> Position {
        board.empty_positions()[0]
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
}

