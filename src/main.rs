#[cfg(test)]
#[macro_use]
extern crate more_asserts;

mod board;
mod strategies;
mod game;

use crate::game::*;
use crate::strategies::*;

fn main() {
    let x = RandomPlayer{};
    let o = RandomPlayer{};
    let game = Game::new(&x, &o);
    println! ("Result {:?}", game.result());
}
