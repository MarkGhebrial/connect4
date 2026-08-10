use crate::{
    algo::evaluate::evaluate,
    connect4::{board::Board, r#move::Move},
};

/// Search for the best move from this board position
pub fn search_moves(board: &Board, depth: usize) -> (Move, i32) {
    let mut best: Option<(Move, i32)> = None;
    for move_ in board.iter_moves() {
        let score = -negamax(&board.clone().with_move(move_), depth - 1);

        // Find the move with the best score
        if best.is_none() || score > best.unwrap().1 {
            best = Some((move_, score));
        }
    }

    best.expect("Tried to search on a board with no legal moves")
}

/// Evaluate a board state using a negamax search with the specified depth. Currently, no alpha-beta
/// pruning is implemented.
///
/// https://chessprogramming.org/Negamax
fn negamax(board: &Board, remaining_depth: usize) -> i32 {
    if remaining_depth == 0 || board.has_four_in_a_row().is_some() {
        return evaluate(board);
    }

    let mut max_score = i32::MIN;
    for r#move in board.iter_moves() {
        let board: Board = board.clone().with_move(r#move);
        // This recursive call tells us how good this new board position is for the opponent. We
        // negate it because a bad score for them is a good score for us.
        let score = -negamax(&board, remaining_depth - 1);
        if score > max_score {
            max_score = score
        }
    }

    max_score
}
