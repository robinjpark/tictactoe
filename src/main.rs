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
    display_result(game, human_token);
}

fn get_player() -> Token {
    println!("You cannot win!");
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

fn display_result(game: Game, human_token: Token) {
    match game.result() {
        GameResult::Draw => println!("You cannot beat me!"),
        GameResult::InProgress => panic!("Should not happen!"),
        GameResult::Win(winner) => {
            if winner == human_token {
                panic!("What??   I cannot lose!!!");
            } else {
                println!("Ha!  I beat you!");
            }
        }
    }
}
