#[cfg(test)]
#[macro_use]
extern crate more_asserts;

mod board;
mod strategies;
mod game;
mod optimal;
mod human;

use crate::board::*;
use crate::strategies::*;
use crate::game::*;
use crate::human::*;
use crate::optimal::*;

fn main() {
    println!("Tic-Tac-Toe");
    println!("In this version, X always plays first.");

    let human_token = get_player();
    let x: Box<dyn Player> = if human_token == Token::X { Box::new(HumanPlayer::new()) } else { Box::new(OptimalPlayer{}) };
    let o: Box<dyn Player> = if human_token == Token::X { Box::new(OptimalPlayer{}) } else { Box::new(HumanPlayer::new()) };
    let game = Game::new(x, o);
    display_result(game, human_token);
}

fn get_player() -> Token {
    println!("You cannot win!");
    println!("Do you want to be X or O?");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("error getting input");
        let input = input.trim();
        let token = match input {
            "X" | "x" => Token::X,
            "O" | "o" => Token::O,
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
