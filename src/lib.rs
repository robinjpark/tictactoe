//! The tictactoe library crate.
//!
//! It implements the entire application.
//!
//! Although this library is only meant for the single tictactoe binary,
//! the application is split into a binary and library to overcome
//! limitations on doc-tests, which can only run in library crates.
#[cfg(test)]
#[macro_use]
extern crate more_asserts;

// Modules needed for benchmarking are public
pub mod board;
mod game;
mod human;
pub mod optimal;
pub mod player;
#[cfg(test)]
pub mod strategies; // Only for unit testing

use crate::board::{GameResult, Token};
use crate::game::Game;
use crate::human::HumanPlayer;
use crate::optimal::OptimalPlayer;

/// The entry point for the "library", which implements the game.
pub fn main() {
    println!("Tic-Tac-Toe");
    println!("In this version, X always plays first.");

    let human_token = get_player();

    let human = HumanPlayer::new();
    let computer = OptimalPlayer {};

    let game = if human_token == Token::X {
        Game::new(human, computer)
    } else {
        Game::new(computer, human)
    };

    display_result(game, human_token);
}

fn get_player() -> Token {
    println!("You cannot win!");
    println!("ᕙ(⇀‸↼‶)ᕗ");
    println!("Do you want to be X or O?");

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("error getting input");
        let input = input.trim();
        let token = match input {
            "X" | "x" => Token::X,
            "O" | "o" => Token::O,
            &_ => {
                println!("Enter 'X' or 'O'!");
                continue;
            }
        };
        return token;
    }
}

fn display_result(game: Game, human_token: Token) {
    match game.result() {
        GameResult::Draw => {
            println!("¯\\_(ツ)_/¯");
            println!("It is a draw?");
            println!("ノಠ益ಠ)ノ彡┻━┻");
        }
        GameResult::InProgress => panic!("Should not happen!"),
        GameResult::Win(winner) => {
            if winner == human_token {
                panic!("What??   I cannot lose!!!");
            } else {
                println!("ᕙ(⇀‸↼‶)ᕗ");
                println!("Ha!  I beat you!");
                println!("ᕙ(⇀‸↼‶)ᕗ");
            }
        }
    }
    println!();
}
