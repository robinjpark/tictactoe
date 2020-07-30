#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Player {
    X,
    O,
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
}
