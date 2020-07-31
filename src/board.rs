//! Utilities for manipulating and querying a tic-tac-toe game board.

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
/// Represents the player of the game (X or O).
enum Player {
    X,
    O,
}

impl Player {
    #[allow(dead_code)]
    #[doc(hidden)]
    fn from_char(value: char) -> Option<Player> {
        if value == 'X' {
            Some(Player::X)
        } else if value == 'O' {
            Some(Player::O)
        } else if value == '-' {
            None
        } else {
            panic!("Invalid character for player: '{}'", value);
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[allow(dead_code)]
/// A position in a tic-tac-toe game board.
struct Position {
    /// row number (0 = top, 2 = bottom)
    row: u8,
    /// column number (0 = left, 2 = right)
    column: u8,
}

impl Position {
    #[allow(dead_code)]
    /// Creates a position given the row and column
    ///
    /// # Panics
    ///
    /// Panics if given row or column is out of range.
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
#[derive(PartialEq, Debug)]
/// Represents a tic-tac-toe game board.
struct Board {
    #[doc(hidden)]
    positions: [[Option<Player>; 3]; 3],
    #[doc(hidden)]
    turn_number: u8,
}

impl Board {
    #[allow(dead_code)]
    /// Creates an empty tic-tac-toe game board.
    ///
    /// Examples
    /// ```
    /// let board = Board::new();
    /// assert_eq!(board.get_game_result(), GameResult::InProgress);
    /// ```
    // TODO: Unforunately, the above example is not checked for correctness!
    // It has something to do with the crate being a binary, not a library.
    fn new() -> Board {
        Board { positions: [[None, None, None],
                            [None, None, None],
                            [None, None, None]],
                turn_number: 1 } // Starts at 1, not 0!
    }

    #[cfg(test)]
    fn from_string(contents: &str) -> Board {
        if contents.len() != 9 {
            panic!("Invalid string length {} for board: '{}'", contents.len(), contents);
        }
        let x_count: i8 = contents.chars().filter(|the_char| *the_char == 'X').collect::<Vec<char>>().len() as i8;
        let o_count: i8 = contents.chars().filter(|the_char| *the_char == 'O').collect::<Vec<char>>().len() as i8;
        let diff_count = x_count - o_count;
        if diff_count != 0 && diff_count != 1 {
            panic!("Invalid number of Xs and Os!");
        }

        let blank_count = contents.chars().filter(|the_char| *the_char == '-').collect::<Vec<char>>().len();
        let mut chars = contents.chars();
        let board = Board { positions: [[Player::from_char(chars.next().unwrap()),
                                         Player::from_char(chars.next().unwrap()),
                                         Player::from_char(chars.next().unwrap())],
                                        [Player::from_char(chars.next().unwrap()),
                                         Player::from_char(chars.next().unwrap()),
                                         Player::from_char(chars.next().unwrap())],
                                        [Player::from_char(chars.next().unwrap()),
                                         Player::from_char(chars.next().unwrap()),
                                         Player::from_char(chars.next().unwrap())]],
                            turn_number: 9 - blank_count as u8 + 1
        };
        board.check_invariants();
        board
    }

    #[allow(dead_code)]
    /// Marks the given position as occupied by the given player.
    ///
    /// # Panics
    ///
    /// Panics if the position is already occupied.
    ///
    /// Panics if the given player is playing out of turn.
    /// Player::X goes first, followed by Player::O, ...
    fn add_move(&mut self, player: Player, at: Position) {
        self.check_invariants();
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
        self.check_invariants();
    }

    #[allow(dead_code)]
    /// Gets the result of the current game.
    fn get_game_result(&self) -> GameResult {
        let mut result = None;
        for row in 0..3 {
            if let Some(player) = self.positions[row][0] {
                if self.positions[row][1] == Some(player) &&
                   self.positions[row][2] == Some(player) {
                       if result != None && result != Some(GameResult::Win(player)) {
                           panic!("Game cannot have multiple winners!");
                       }
                       result = Some(GameResult::Win(player));
                }
            }
        }

        for column in 0..3 {
            if let Some(player) = self.positions[0][column] {
                if self.positions[1][column] == Some(player) &&
                   self.positions[2][column] == Some(player) {
                       if result != None && result != Some(GameResult::Win(player)) {
                           panic!("Game cannot have multiple winners!");
                       }
                       result = Some(GameResult::Win(player));
                }
            }
        }

        // diagonal in direction '\'
        if let Some(player) = self.positions[0][0] {
            if self.positions[1][1] == Some(player) &&
               self.positions[2][2] == Some(player) {
                   result = Some(GameResult::Win(player));
            }
        }

        // diagonal in direction '/'
        if let Some(player) = self.positions[2][0] {
            if self.positions[1][1] == Some(player) &&
               self.positions[0][2] == Some(player) {
               result = Some(GameResult::Win(player));
            }
        }

        if let Some(result) = result {
            result
        } else if self.turn_number <= 9 {
            GameResult::InProgress
        } else {
            GameResult::Draw
        }
    }

    #[allow(dead_code)]
    #[doc(hidden)]
    fn check_invariants(&self) {
        let _winner = self.get_game_result();
        if self.turn_number == 0 || self.turn_number > 10 {
            panic!("Invalid turn number {}!", self.turn_number);
        }
    }
} // impl Board

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Will look something like:
        // =====
        // |X O|
        // | X |
        // |OOX|
        // =====
        write!(f, "=====\n|{}{}{}|\n|{}{}{}|\n|{}{}{}|\n=====",
               match self.positions[0][0] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[0][1] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[0][2] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[1][0] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[1][1] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[1][2] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[2][0] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[2][1] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               },
               match self.positions[2][2] {
                   Some(Player::X) => "X",
                   Some(Player::O) => "O",
                   None => " ",
               })
    } // fn fmt()
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
/// Indicates the result of a game.
enum GameResult {
    /// The given Player has won the game.
    Win(Player),
    /// The game ended in a draw.
    Draw,
    /// The game is still in progress.
    InProgress,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_equality() {
        let player1 = Player::X;
        let player2 = Player::O;
        let player3 = Player::X;
        assert_ne!(player1, player2);
        assert_eq!(player1, player3);
    }

    #[test]
    fn test_player_display() {
        let x = Player::X;
        let o = Player::O;
        assert_eq!(format!("{}", x), "X");
        assert_eq!(format!("{}", o), "O");
        assert_eq!(format!("{}{}", x, o), "XO");
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
    fn test_board_from_string() {
        let full_board = Board::from_string("XOX\
                                             OXO\
                                             XOX");
        assert_eq!(Some(Player::X), full_board.positions[0][0]);
        assert_eq!(10, full_board.turn_number);

        let empty_board = Board::from_string("---\
                                              ---\
                                              ---");
        assert_eq!(None, empty_board.positions[0][0]);
        assert_eq!(1, empty_board.turn_number);
        assert_eq!(Board::new(), empty_board);
    }

    #[test]
    #[should_panic(expected = "Invalid character for player")]
    fn test_invalid_board_from_string_bad_char() {
        let _invalid_player_board = Board::from_string("XOX\
                                                        O O\
                                                        XOX");
    }

    #[test]
    #[should_panic(expected = "Invalid string length 10")]
    fn test_invalid_board_from_string_bad_length() {
        let _invalid_length_board = Board::from_string("XOX\
                                                        OXO\
                                                        XOXO");
    }

    #[test]
    #[should_panic(expected = "Invalid number of Xs and Os")]
    fn test_invalid_board_from_string_bad_ratio() {
        let _invalid_count_board = Board::from_string("XXX\
                                                       ---\
                                                       --O");
    }

    #[test]
    #[should_panic(expected = "Game cannot have multiple winners!")]
    fn test_invalid_board_from_string_multiple_winners() {
        let _multiple_winners_board = Board::from_string("XXX\
                                                          OOO\
                                                          ---");
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
    fn test_board_display() {
        let board = Board::from_string("XOX\
                                        OXO\
                                        OXX");
        assert_eq!(format!("{}", board), "=====\n|XOX|\n|OXO|\n|OXX|\n=====");

        let board = Board::from_string("XOX\
                                        ---\
                                        OOX");
        println!("{}", board);
        assert_eq!(format!("{}", board), "=====\n|XOX|\n|   |\n|OOX|\n=====");
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

        let board = Board::from_string("XXX\
                                        OO-\
                                        ---");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::X));

        let board = Board::from_string("XX-\
                                        OOO\
                                        X--");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::O));

        let board = Board::from_string("XX-\
                                        X--\
                                        OOO");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::O));

        let board = Board::from_string("XO-\
                                        XO-\
                                        X--");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::X));

        let board = Board::from_string("XO-\
                                        -O-\
                                        XOX");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::O));

        let board = Board::from_string("-OX\
                                        --X\
                                        -OX");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::X));

        let board = Board::from_string("X-O\
                                        -X-\
                                        O-X");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::X));

        let board = Board::from_string("X-O\
                                        XO-\
                                        O-X");
        assert_eq!(board.get_game_result(), GameResult::Win(Player::O));
    } // test_winning_game()

    #[test]
    fn test_no_winner_game() {
        let board = Board::from_string("XOX\
                                        XXO\
                                        OXO");
        assert_eq!(board.get_game_result(), GameResult::Draw);
    }

    #[test]
    fn test_in_progress_game() {
        let board = Board::new();
        assert_eq!(board.get_game_result(), GameResult::InProgress);

        let board = Board::from_string("XOX\
                                        X-O\
                                        OXO");
        assert_eq!(board.get_game_result(), GameResult::InProgress);
    }
}
