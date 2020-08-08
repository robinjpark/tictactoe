use crate::board::{Board, Position};
use crate::player::Player;

use std::io::{BufRead, Write};

pub struct HumanPlayer<'a> {
    reader: &'a mut (dyn BufRead + 'a),
    writer: &'a mut (dyn Write + 'a),
}

impl<'a> HumanPlayer<'a> {
    pub fn new(
        reader: &'a mut (dyn BufRead + 'a),
        writer: &'a mut (dyn Write + 'a),
    ) -> HumanPlayer<'a> {
        HumanPlayer::print_instructions(writer);
        HumanPlayer { reader, writer }
    }

    fn print_instructions(writer: &'a mut (dyn Write + 'a)) {
        writeln!(writer).unwrap();
        writeln!(writer, "Instructions...").unwrap();
        writeln!(
            writer,
            "I will query for a number between 1..9 for each move."
        )
        .unwrap();
        writeln!(writer, "The numbers correspond to the following diagram:").unwrap();
        writeln!(writer, "┌───┐").unwrap();
        writeln!(writer, "│123│").unwrap();
        writeln!(writer, "│456│").unwrap();
        writeln!(writer, "│789│").unwrap();
        writeln!(writer, "└───┘").unwrap();
        writeln!(writer).unwrap();
    }
}

impl<'a> Player for HumanPlayer<'a> {
    fn take_turn(&mut self, board: &Board) -> Position {
        writeln!(self.writer, "{}", board).unwrap();
        loop {
            writeln!(self.writer, "Where would you like to go? (1-9)").unwrap();
            let mut input = String::new();
            self.reader
                .read_line(&mut input)
                .expect("error getting input");
            let input = input.trim();
            let position = match input {
                "1" => Position::new(0, 0),
                "2" => Position::new(0, 1),
                "3" => Position::new(0, 2),
                "4" => Position::new(1, 0),
                "5" => Position::new(1, 1),
                "6" => Position::new(1, 2),
                "7" => Position::new(2, 0),
                "8" => Position::new(2, 1),
                "9" => Position::new(2, 2),
                &_ => {
                    writeln!(self.writer, "That is not a valid position!").unwrap();
                    continue;
                }
            };

            if board.is_position_unused(position) {
                return position;
            } else {
                writeln!(self.writer, "That position is already occupied!").unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_input_output() {
        let mut turn_input = b"5\n" as &[u8];
        let mut turn_output: Vec<u8> = Vec::new();
        let mut human = HumanPlayer::new(&mut turn_input, &mut turn_output);

        let board = Board::new();
        let position = human.take_turn(&board);

        assert_eq!(position, Position::new(1, 1));

        let output: &str = str::from_utf8(&turn_output).unwrap();
        let output = String::from(output);
        let should_contain = "Where would you like to go?";
        assert_eq!(
            output.contains(should_contain),
            true,
            "\nOutput did not contain '{}'\nOutput was:\n{}'",
            should_contain,
            output
        );
    }
}
