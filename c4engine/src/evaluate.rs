use c4board::{board::Board, player_color::PlayerColor};

/// Given a board position, approximate which player has the advantage and by how much. Positive
/// evaluations mean the up next player has an advantage and negative evaluations mean the up next
/// player has a disadvantage.
pub fn evaluate(board: &Board) -> i32 {
    let mut yellow_score: i32 = 0;
    let mut red_score: i32 = 0;

    if let Some(player) = board.winner() {
        match player {
            PlayerColor::Yellow => yellow_score += 400,
            PlayerColor::Red => red_score += 400,
        }
    }

    match board.up_next() {
        PlayerColor::Yellow => yellow_score - red_score,
        PlayerColor::Red => red_score - yellow_score,
    }
}
