use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tictactoelib::optimal::OptimalPlayer;
use tictactoelib::board::{Board, Position, Token};
use tictactoelib::player::Player;

pub fn optimal_player_benchmark(c: &mut Criterion) {
    let player = OptimalPlayer{};
    let mut board = Board::new();
    board.add_move(Token::X, Position::new(1,1)); // center
    c.bench_function("optimal_player: turn #2", |b| b.iter(|| player.take_turn(black_box(&board))));
}

criterion_group!(benches, optimal_player_benchmark);
criterion_main!(benches);
