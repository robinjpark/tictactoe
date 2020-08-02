//! Utilities for manipulating and querying a tic-tac-toe game board.

#[derive(Copy, Clone, Debug, PartialEq)]
/// Represents the player of the game (X or O).
pub enum Token {
    X,
    O,
}

impl Token {
    #[cfg(test)]
    #[doc(hidden)]
    fn from_char(value: char) -> Option<Token> {
        if value == 'X' {
            Some(Token::X)
        } else if value == 'O' {
            Some(Token::O)
        } else if value == '-' {
            None
        } else {
            panic!("Invalid character for player: '{}'", value);
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Token::X => write!(f, "X"),
            Token::O => write!(f, "O"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// A position in a tic-tac-toe game board.
pub struct Position {
    /// row number (0 = top, 2 = bottom)
    row: u8,
    /// column number (0 = left, 2 = right)
    column: u8,
}

impl Position {
    /// Creates a position given the row and column
    ///
    /// # Panics
    ///
    /// Panics if given row or column is out of range.
    pub fn new(row: u8, column: u8) -> Position {
        if row > 2 {
            panic!("Invalid row: {}", row);
        }
        if column > 2 {
            panic!("Invalid column: {}", column);
        }
        Position{ row, column }
    }
}

#[derive(PartialEq, Debug)]
/// Represents a tic-tac-toe game board.
pub struct Board {
    #[doc(hidden)]
    positions: [[Option<Token>; 3]; 3],
    #[doc(hidden)]
    turn_number: u8,
}

impl Board {
    /// Creates an empty tic-tac-toe game board.
    ///
    /// Examples
    /// ```
    /// let board = Board::new();
    /// assert_eq!(board.get_game_result(), GameResult::InProgress);
    /// ```
    // TODO: Unforunately, the above example is not checked for correctness!
    // It has something to do with the crate being a binary, not a library.
    pub fn new() -> Board {
        Board { positions: [[None, None, None],
                            [None, None, None],
                            [None, None, None]],
                turn_number: 1 } // Starts at 1, not 0!
    }

    #[cfg(test)]
    pub fn from_string(contents: &str) -> Board {
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
        let board = Board { positions: [[Token::from_char(chars.next().unwrap()),
                                         Token::from_char(chars.next().unwrap()),
                                         Token::from_char(chars.next().unwrap())],
                                        [Token::from_char(chars.next().unwrap()),
                                         Token::from_char(chars.next().unwrap()),
                                         Token::from_char(chars.next().unwrap())],
                                        [Token::from_char(chars.next().unwrap()),
                                         Token::from_char(chars.next().unwrap()),
                                         Token::from_char(chars.next().unwrap())]],
                            turn_number: 9 - blank_count as u8 + 1
        };
        board.check_invariants();
        board
    }

    /// Returns all of the empty positions in a vector
    pub fn empty_positions(&self) -> Vec<Position> {
        let mut vec = Vec::new();
        for row in 0..3 {
            for column in 0..3 {
                if self.positions[row][column] == None {
                    vec.push(Position::new(row as u8, column as u8));
                }
            }
        }
        vec
    }

    /// Marks the given position as occupied by the given player.
    ///
    /// # Panics
    ///
    /// Panics if the position is already occupied.
    ///
    /// Panics if the given player is playing out of turn.
    /// Token::X goes first, followed by Token::O, ...
    pub fn add_move(&mut self, player: Token, at: Position) {
        self.check_invariants();
        if player == Token::X && self.turn_number % 2 == 0 {
            panic!("It is not X's turn!");
        }
        if player == Token::O && self.turn_number % 2 == 1 {
            panic!("It is not O's turn!");
        }
        if let Some(_player) = self.positions[at.row as usize][at.column as usize] {
            panic!("Position [{},{}] is already occupied!", at.row, at.column);
        }
        self.positions[at.row as usize][at.column as usize] = Some(player);
        self.turn_number += 1;
        self.check_invariants();
    }

    /// Gets the result of the current game.
    pub fn get_game_result(&self) -> GameResult {
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
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[0][1] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[0][2] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[1][0] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[1][1] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[1][2] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[2][0] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[2][1] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               },
               match self.positions[2][2] {
                   Some(Token::X) => "X",
                   Some(Token::O) => "O",
                   None => " ",
               })
    } // fn fmt()
} // impl std::fmt::Display for Board

#[derive(Debug, PartialEq)]
/// Indicates the result of a game.
pub enum GameResult {
    /// The given Token has won the game.
    Win(Token),
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
        let player1 = Token::X;
        let player2 = Token::O;
        let player3 = Token::X;
        assert_ne!(player1, player2);
        assert_eq!(player1, player3);
    }

    #[test]
    fn test_player_display() {
        let x = Token::X;
        let o = Token::O;
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
        assert_eq!(Some(Token::X), full_board.positions[0][0]);
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
    fn test_empty_positions_full_board() {
        let full_board = Board::from_string("XOX\
                                             OOX\
                                             XXO");
        assert_eq!(full_board.empty_positions(), Vec::new());
    }

    #[test]
    fn test_empty_positions_one_position_empty() {
        let one_left = Board::from_string("XOX\
                                           OO-\
                                           XXO");
        assert_eq!(one_left.empty_positions(), vec![Position::new(1,2)]);
    }

    #[test]
    fn test_empty_positions_two_positions_empty() {
        let two_left = Board::from_string("X-X\
                                           OOX\
                                           -XO");
        assert_eq!(two_left.empty_positions(), vec![Position::new(0,1), Position::new(2,0)]);
    }

    #[test]
    fn test_empty_positions_empty_board() {
        let two_left = Board::new();
        assert_eq!(two_left.empty_positions(), vec![Position::new(0,0), Position::new(0,1), Position::new(0,2),
                                                    Position::new(1,0), Position::new(1,1), Position::new(1,2),
                                                    Position::new(2,0), Position::new(2,1), Position::new(2,2)]);
    }

    #[test]
    fn test_add_to_board() {
        let mut board = Board::new();

        board.add_move(Token::X, Position::new(0, 0));
        assert_eq!(Some(Token::X), board.positions[0][0]);
        assert_eq!(2, board.turn_number);

        board.add_move(Token::O, Position::new(1, 1));
        assert_eq!(Some(Token::O), board.positions[1][1]);
        assert_eq!(3, board.turn_number);

        board.add_move(Token::X, Position::new(0, 1));
        board.add_move(Token::O, Position::new(0, 2));
        board.add_move(Token::X, Position::new(1, 0));
        board.add_move(Token::O, Position::new(1, 2));
        board.add_move(Token::X, Position::new(2, 0));
        board.add_move(Token::O, Position::new(2, 1));
        assert_eq!(9, board.turn_number);
        board.add_move(Token::X, Position::new(2, 2));

        assert_eq!(Some(Token::X), board.positions[0][1]);
        assert_eq!(Some(Token::X), board.positions[1][0]);
        assert_eq!(Some(Token::X), board.positions[2][0]);
        assert_eq!(Some(Token::X), board.positions[2][2]);

        assert_eq!(Some(Token::O), board.positions[0][2]);
        assert_eq!(Some(Token::O), board.positions[1][2]);
        assert_eq!(Some(Token::O), board.positions[2][1]);

    }

    #[test]
    #[should_panic(expected = "Position [1,2] is already occupied!")]
    fn test_overwrite_position() {
        let mut board = Board::new();

        board.add_move(Token::X, Position::new(1, 2));
        board.add_move(Token::O, Position::new(1, 2));
    }

    #[test]
    #[should_panic(expected = "It is not O's turn!")]
    fn test_wrong_starting_player() {
        let mut board = Board::new();

        board.add_move(Token::O, Position::new(1, 1));
    }

    #[test]
    #[should_panic(expected = "It is not X's turn!")]
    fn test_wrong_players_turn() {
        let mut board = Board::new();

        board.add_move(Token::X, Position::new(1, 1));
        board.add_move(Token::X, Position::new(2, 2));
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

        board.add_move(Token::X, Position::new(0, 0));
        board.add_move(Token::O, Position::new(1, 0));
        board.add_move(Token::X, Position::new(0, 1));
        board.add_move(Token::O, Position::new(1, 1));
        board.add_move(Token::X, Position::new(0, 2));
        assert_eq!(board.get_game_result(), GameResult::Win(Token::X));

        let board = Board::from_string("XXX\
                                        OO-\
                                        ---");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::X));

        let board = Board::from_string("XX-\
                                        OOO\
                                        X--");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::O));

        let board = Board::from_string("XX-\
                                        X--\
                                        OOO");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::O));

        let board = Board::from_string("XO-\
                                        XO-\
                                        X--");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::X));

        let board = Board::from_string("XO-\
                                        -O-\
                                        XOX");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::O));

        let board = Board::from_string("-OX\
                                        --X\
                                        -OX");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::X));

        let board = Board::from_string("X-O\
                                        -X-\
                                        O-X");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::X));

        let board = Board::from_string("X-O\
                                        XO-\
                                        O-X");
        assert_eq!(board.get_game_result(), GameResult::Win(Token::O));
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
} // mod tests
