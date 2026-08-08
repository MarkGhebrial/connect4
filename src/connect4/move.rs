use crate::connect4::bitboard;

/// A game move. Connect four moves are fully defined by which column you put the checker in, so
/// this struct has exactly one field, indicating which column was played.
pub struct Move {
    column: u8,
}

impl Move {
    pub fn new(col_idx: u8) -> Self {
        assert!(col_idx < bitboard::NUM_COLUMNS as u8);
        Self { column: col_idx }
    }
}
