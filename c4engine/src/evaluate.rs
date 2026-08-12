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

    yellow_score += (board.yellow() & Bitboard::NTH_COLUMN[3]).count_ones() as i32;
    // red_score += (board.red() & Bitboard::NTH_COLUMN[3]).count_ones() as i32;

    if let Some(player) = board.has_four_in_a_row() {
        match player {
            PlayerColor::Yellow => yellow_score += 400,
            PlayerColor::Red => red_score += 400,
        }
    }

    // for four in Bitboard::FOURS {
    //     // yellow_score += (board.yellow() & four).count_ones() as i32;
    //     // red_score += (board.red() & four).count_ones() as i32;
    // }

    // println!("yellow {} red {}", yellow_score, red_score);

    match board.up_next() {
        PlayerColor::Yellow => yellow_score - red_score,
        PlayerColor::Red => red_score - yellow_score,
    }
}

// // Given a bitboard of the up next player's pieces and a bitboard of the other player's pieces,
// // return an evaluation for how strong the player's position is.
// fn evaluate_position(up_next_bb: Bitboard, other_bb: Bitboard) -> i32 {
//     todo!()
// }
