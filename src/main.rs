#[cfg(test)]
#[macro_use]
extern crate more_asserts;

mod board;
mod strategies;
mod game;
mod optimal;
mod human;

use crate::game::*;
use crate::strategies::*;
use crate::human::*;

fn main() {
    let stdio = std::io::stdin();
    let input = stdio.lock();
    let output = std::io::stdout();
    let mut x = HumanPlayer { reader: input, writer: output };
    let mut o = RandomPlayer{};
    let game = Game::new(&mut x, &mut o);
    println! ("Result {:?}", game.result());
}
