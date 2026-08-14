//! This module contains the actual algorithm for searching the game tree and for evaluating moves.

mod evaluate;
mod search;

use std::io::{stdin, stdout};

use c4board::{board::Board, r#move::Move};
use c4i::C4IServer;
use search::search_moves;

pub struct MyC4IServer;
impl c4i::C4IServer for MyC4IServer {
    fn play(board: &Board) -> Move {
        search_moves(board, 4).0
    }
}

fn main() {
    <MyC4IServer as C4IServer>::start_server(&mut stdin(), &mut stdout()).unwrap();
}
