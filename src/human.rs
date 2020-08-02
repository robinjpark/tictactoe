use crate::board::*;
use crate::strategies::*;
use std::io::{self, BufRead, Write};

// TODO: Make reader and writer private, initialize them with a ctor fn
pub struct HumanPlayer<R,W> {
    pub reader: R,
    pub writer: W
}

impl<R,W> Player for HumanPlayer<R,W>
where
    R: BufRead,
    W: Write,
{
    fn take_turn(&mut self, board: &Board) -> Position {
        writeln!(&mut self.writer, "{}", board);
        loop {
            writeln!(&mut self.writer, "Where would you like to go? (1-9)");
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("error getting input");
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
                    writeln!(&mut self.writer, "That is not a valid position!");
                    continue;
                }
            };

            if board.is_empty(position) {
                return position;
            } else {
                writeln!(&mut self.writer, "That position is already occupied!");
            }
        }
    }
}

