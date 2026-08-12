use crate::bitboard;

/// A game move. Connect four moves are fully defined by which column you put the checker in, so
/// this struct has exactly one field, indicating which column was played.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    column: u8,
}

impl Move {
    pub fn new(col_idx: u8) -> Self {
        assert!(col_idx < bitboard::NUM_COLUMNS as u8);
        Self { column: col_idx }
    }

    pub fn column(&self) -> u8 {
        self.column
    }
}
