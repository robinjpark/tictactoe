use crate::board::*;
use crate::strategies::*;
use std::io;

pub struct HumanPlayer {
}

impl Player for HumanPlayer {
    fn take_turn(&self, board: &Board) -> Position {
        println!("{}", board);
        println!("Where would you like to go? (1-9)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error getting input");
        let input = input.trim();
        let position = match input {
            "1" => Some(Position::new(0,0)),
            "2" => Some(Position::new(0,1)),
            "3" => Some(Position::new(0,2)),
            "4" => Some(Position::new(1,0)),
            "5" => Some(Position::new(1,1)),
            "6" => Some(Position::new(1,2)),
            "7" => Some(Position::new(2,0)),
            "8" => Some(Position::new(2,1)),
            "9" => Some(Position::new(2,2)),
            &_ => {
                println!("That is not a valid position!");
                None
            }
        };

        position.unwrap()
    }
}

