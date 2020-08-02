use crate::board::*;
use crate::strategies::*;
use std::io;

pub struct HumanPlayer {
}

impl Player for HumanPlayer {
    fn take_turn(&self, board: &Board) -> Position {
        println!("{}", board);
        loop {
            println!("Where would you like to go? (1-9)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("error getting input");
            let input = input.trim();
            let position = match input {
                "1" => Position::new(0,0),
                "2" => Position::new(0,1),
                "3" => Position::new(0,2),
                "4" => Position::new(1,0),
                "5" => Position::new(1,1),
                "6" => Position::new(1,2),
                "7" => Position::new(2,0),
                "8" => Position::new(2,1),
                "9" => Position::new(2,2),
                &_ => {
                    println!("That is not a valid position!");
                    continue;
                }
            };

            return position;
        }
    }
}

