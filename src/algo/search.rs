use crate::{algo::evaluate::evaluate, connect4::board::Board};

/// Evaluate a board state using a negamax search with the specified depth. Currently, no alpha-beta
/// pruning is implemented.
///
/// https://chessprogramming.org/Negamax
pub fn negamax(board: &Board, remaining_depth: usize) -> i32 {
    if remaining_depth == 0 {
        return evaluate(board);
    }

    let mut max_score = i32::MIN;
    for r#move in board.iter_moves() {
        let mut board: Board = board.clone();
        board.apply_move(r#move);
        let score = -negamax(&board, remaining_depth - 1);
        if score > max_score {
            max_score = score
        }
    }

    max_score
}
