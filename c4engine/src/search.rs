use crate::evaluate::evaluate;

use c4board::{board::Board, r#move::Move};

/// Search for the best move from this board position. Returns None if there are no legal moves.
pub fn search_moves(board: &Board, depth: usize) -> Option<(Move, i32)> {
    let mut best: Option<(Move, i32)> = None;
    for move_ in board.iter_moves() {
        let score = -negamax(
            &board.clone().with_move(move_),
            i32::MIN,
            i32::MAX,
            depth - 1,
        );

        // Find the move with the best score
        if best.is_none() || score > best.unwrap().1 {
            best = Some((move_, score));
        }
    }

    best
}

/// Evaluate a board state using a negamax search with the specified depth.
///
/// https://chessprogramming.org/Negamax
/// https://chessprogramming.org/Alpha-Beta
fn negamax(board: &Board, mut alpha: i32, beta: i32, remaining_depth: usize) -> i32 {
    if remaining_depth == 0 || board.is_game_over() {
        return evaluate(board);
    }

    let mut max_score = i32::MIN;
    for r#move in board.iter_moves() {
        let board: Board = board.clone().with_move(r#move);
        // This recursive call tells us how good this new board position is for the opponent. We
        // negate it because a bad score for them is a good score for us.
        let score = -negamax(&board, -beta, -alpha, remaining_depth - 1);
        if score > max_score {
            max_score = score;
            if score > alpha {
                alpha = score;
            }
        }
        // Don't search the rest of the moves if this move is better than ??? TODO: Understand alpha/beta search well enough to write this comment
        if score >= beta {
            return max_score;
        }
    }

    max_score
}
