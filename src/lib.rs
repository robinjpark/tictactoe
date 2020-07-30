#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
}

impl Board {
    #[allow(dead_code)]
    fn new() -> Board {
        Board { positions: [[None, None, None],
                            [None, None, None],
                            [None, None, None]]}
    }

    #[allow(dead_code)]
    fn add_move(&mut self, player: Player, at: Position) {
        self.positions[at.row as usize][at.column as usize] = Some(player);
    }
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

        board.add_move(Player::O, Position::new(1, 1));
        assert_eq!(Some(Player::O), board.positions[1][1]);

        board.add_move(Player::X, Position::new(0, 1));
        board.add_move(Player::O, Position::new(0, 2));
        board.add_move(Player::X, Position::new(1, 0));
        board.add_move(Player::O, Position::new(1, 2));
        board.add_move(Player::X, Position::new(2, 0));
        board.add_move(Player::O, Position::new(2, 1));
        board.add_move(Player::X, Position::new(2, 2));

        assert_eq!(Some(Player::X), board.positions[0][1]);
        assert_eq!(Some(Player::X), board.positions[1][0]);
        assert_eq!(Some(Player::X), board.positions[2][0]);
        assert_eq!(Some(Player::X), board.positions[2][2]);

        assert_eq!(Some(Player::O), board.positions[0][2]);
        assert_eq!(Some(Player::O), board.positions[1][2]);
        assert_eq!(Some(Player::O), board.positions[2][1]);
    }
}
