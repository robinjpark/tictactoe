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
    let x = HumanPlayer{};
    let o = RandomPlayer{};
    let game = Game::new(&x, &o);
    println! ("Result {:?}", game.result());
}
