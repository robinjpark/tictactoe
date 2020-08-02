#[cfg(test)]
#[macro_use]
extern crate more_asserts;

mod board;
mod strategies;
mod game;
mod optimal;
mod human;

use crate::game::*;
use crate::human::*;
use crate::optimal::*;

fn main() {
    let x = HumanPlayer{};
    let o = OptimalPlayer{};
    let game = Game::new(&x, &o);
    println! ("Result {:?}", game.result());
}
