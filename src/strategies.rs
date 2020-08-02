use crate::board::{Position, Board};
#[cfg(test)]
use rand::Rng;

pub trait Player {
    fn take_turn (&self, board: &Board) -> Position;
}

#[cfg(test)]
pub struct RandomPlayer {
}

#[cfg(test)]
impl Player for RandomPlayer {
    fn take_turn (&self, board: &Board) -> Position
    {
        let mut rng = rand::thread_rng();

        let empty_positions = board.empty_positions();
        let count = empty_positions.len();
        let position_to_choose = rng.gen_range(0, count);
        empty_positions[position_to_choose]
    }
}

#[cfg(test)]
mod random_player_tests {
    use super::*;
    use crate::board::{GameResult, Token};

    #[test]
    fn test_random_played_boards_differ() {
        let player = RandomPlayer{};
        //for _n in 1..1000 { // Uncomment to tune MAX_SAME_GAMES
        const MAX_GAMES:u32 = 100;
        const MAX_SAME_GAMES:u32 = 35; // Determined by running this many many times!
        println!("Play {} games with players that move randomly.", MAX_GAMES);
        println!("The games should be different!");
        let mut games = Vec::<Board>::new();
        for game in 1..MAX_GAMES+1 {
            let mut board = Board::new();
            for turn in 1..10 {
                let position = player.take_turn(&board);
                let token =
                    if turn % 2 == 1 {
                        Token::X
                    } else {
                        Token::O
                    };
                board.add_move(token, position);
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
} // random_player_tests
