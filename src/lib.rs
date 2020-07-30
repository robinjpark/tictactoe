#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    X,
    O,
}

#[allow(dead_code)]
struct Position {
    row: u8,
    column: u8,
}

impl Position {
    #[allow(dead_code)]
    fn new(row: u8, column: u8) -> Position {
        if row > 2 {
            panic!("Invalid row: {}", row);
        }
        if column > 2 {
            panic!("Invalid column: {}", column);
        }
        Position{ row, column }
    }
}

#[allow(dead_code)]
struct Board {
    positions: [[Option<Player>; 3]; 3],
    turn_number: u8,
}

impl Board {
    #[allow(dead_code)]
    fn new() -> Board {
        Board { positions: [[None, None, None],
                            [None, None, None],
                            [None, None, None]],
                turn_number: 1 } // Starts at 1, not 0!
    }

    #[allow(dead_code)]
    fn add_move(&mut self, player: Player, at: Position) {
        if player == Player::X && self.turn_number % 2 == 0 {
            panic!("It is not X's turn!");
        }
        if player == Player::O && self.turn_number % 2 == 1 {
            panic!("It is not O's turn!");
        }
        if let Some(_player) = self.positions[at.row as usize][at.column as usize] {
            panic!("Position [{},{}] is already occupied!", at.row, at.column);
        }
        self.positions[at.row as usize][at.column as usize] = Some(player);
        self.turn_number += 1;
    }

    #[allow(dead_code)]
    fn get_game_result(self) -> GameResult {
        if let Some(player) = self.positions[0][0] {
            if self.positions[0][1] == Some(player) &&
               self.positions[0][2] == Some(player) {
                   return GameResult::Win(player)
            }
        }

        panic!("Finish me!!!");
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum GameResult {
    Win(Player),
    Draw,
    InProgress,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player() {
        let player1 = Player::X;
        let player2 = Player::O;
        let player3 = Player::X;
        assert_ne!(player1, player2);
        assert_eq!(player1, player3);
    }

    #[test]
    fn test_position_ctor_success() {
        assert_eq!(Position::new(0, 0).row, 0);
        assert_eq!(Position::new(0, 0).column, 0);

        assert_eq!(Position::new(1, 2).row, 1);
        assert_eq!(Position::new(1, 2).column, 2);
    }

    #[test]
    #[should_panic(expected = "Invalid row: 3")]
    fn test_position_ctor_invalid_row() {
        let _position = Position::new(3, 0);
    }

    #[test]
    #[should_panic(expected = "Invalid column: 3")]
    fn test_position_ctor_invalid_column() {
        let _position = Position::new(0, 3);
    }

    #[test]
    fn test_empty_board() {
        let empty = Board::new();
        for row in 0..2 {
            for column in 0..2 {
                assert_eq!(None, empty.positions[row][column]);
            }
        }
    }

    #[test]
    fn test_add_to_board() {
        let mut board = Board::new();

        board.add_move(Player::X, Position::new(0, 0));
        assert_eq!(Some(Player::X), board.positions[0][0]);
        assert_eq!(2, board.turn_number);

        board.add_move(Player::O, Position::new(1, 1));
        assert_eq!(Some(Player::O), board.positions[1][1]);
        assert_eq!(3, board.turn_number);

        board.add_move(Player::X, Position::new(0, 1));
        board.add_move(Player::O, Position::new(0, 2));
        board.add_move(Player::X, Position::new(1, 0));
        board.add_move(Player::O, Position::new(1, 2));
        board.add_move(Player::X, Position::new(2, 0));
        board.add_move(Player::O, Position::new(2, 1));
        assert_eq!(9, board.turn_number);
        board.add_move(Player::X, Position::new(2, 2));

        assert_eq!(Some(Player::X), board.positions[0][1]);
        assert_eq!(Some(Player::X), board.positions[1][0]);
        assert_eq!(Some(Player::X), board.positions[2][0]);
        assert_eq!(Some(Player::X), board.positions[2][2]);

        assert_eq!(Some(Player::O), board.positions[0][2]);
        assert_eq!(Some(Player::O), board.positions[1][2]);
        assert_eq!(Some(Player::O), board.positions[2][1]);

    }

    #[test]
    #[should_panic(expected = "Position [1,2] is already occupied!")]
    fn test_overwrite_position() {
        let mut board = Board::new();

        board.add_move(Player::X, Position::new(1, 2));
        board.add_move(Player::O, Position::new(1, 2));
    }

    #[test]
    #[should_panic(expected = "It is not O's turn!")]
    fn test_wrong_starting_player() {
        let mut board = Board::new();

        board.add_move(Player::O, Position::new(1, 1));
    }

    #[test]
    #[should_panic(expected = "It is not X's turn!")]
    fn test_wrong_players_turn() {
        let mut board = Board::new();

        board.add_move(Player::X, Position::new(1, 1));
        board.add_move(Player::X, Position::new(2, 2));
    }

    #[test]
    fn test_winning_game() {
        let mut board = Board::new();

        board.add_move(Player::X, Position::new(0, 0));
        board.add_move(Player::O, Position::new(1, 0));
        board.add_move(Player::X, Position::new(0, 1));
        board.add_move(Player::O, Position::new(1, 1));
        board.add_move(Player::X, Position::new(0, 2));
        assert_eq!(board.get_game_result(), GameResult::Win(Player::X));
    }
}
