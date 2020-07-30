#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Player {
    X,
    O,
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
    fn test_empty_board() {
        let empty = Board::new();
        for row in 0..2 {
            for column in 0..2 {
                assert_eq!(None, empty.positions[row][column]);
            }
        }
    }
}
