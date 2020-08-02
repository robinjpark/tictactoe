#[cfg(test)]
#[macro_use]
extern crate more_asserts;

mod board;
mod strategies;
mod game;
mod optimal;
mod human;

use crate::board::*;
use crate::game::*;
use crate::human::*;
use crate::optimal::*;

fn main() {
    println!("Tic-Tac-Toe");
    println!("In this version, X always plays first.");

    let human_token = get_player();
    let human = HumanPlayer{};
    let computer = OptimalPlayer{};

    let game = if human_token == Token::X { Game::new(&human, &computer) } else  { Game::new(&computer, &human) };
    println! ("Result {:?}", game.result());
}

fn get_player() -> Token {
    println!("Do you want to be X or O?");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("error getting input");
        let input = input.trim();
        println!("You entered '{}'", input);
        let token = match input {
            "X" => Token::X,
            "O" => Token::O,
            &_ => {
                println!("Enter 'X' or 'O'!");
                continue;
            },
        };
        return token;
    }
}
