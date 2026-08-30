use c4board::{bitboard::Bitboard, board::Board, player_color::PlayerColor};

/// Given a board position, approximate which player has the advantage and by how much. Positive
/// evaluations mean the up next player has an advantage and negative evaluations mean the up next
/// player has a disadvantage.
pub fn evaluate(board: &Board) -> i32 {
    // Reward having four in a row
    // Reward having three in a row
    // Reward having two in a row

    //

    let mut yellow_score: i32 = 0;
    let mut red_score: i32 = 0;

    // TODO: This doesn't seem to do anything at all to the engine strength. Probably should be removed
    let column_weights = [0, 1, 2, 3, 2, 1, 0];
    for (column_idx, column_weight) in column_weights.into_iter().map(|w| w * 100).enumerate() {
        yellow_score +=
            column_weight * (board.yellow() & Bitboard::NTH_COLUMN[column_idx]).count_ones() as i32;
        red_score +=
            column_weight * (board.red() & Bitboard::NTH_COLUMN[column_idx]).count_ones() as i32;
    }

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
