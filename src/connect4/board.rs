use std::debug_assert;

use crate::connect4::{
    bitboard::{Bitboard, NUM_COLUMNS},
    r#move::Move,
    player_color::PlayerColor,
};

/// A possible state of the board. The positions of the yellow and red checkers are represented as a
/// bitboards
/// where 0 is the least significant bit. Unlike chess, we don't have a perfect 8x8 grid, so every
/// bitboard has 22 wasted bits at the end.
#[derive(Debug, Clone, Copy)]
pub struct Board {
    /// The positions of the yellow checkers. Yellow is the player that goes first.
    yellow: Bitboard,
    /// The positions of the red checkers
    red: Bitboard,
    /// The player whose move it is. Even number of checkers
    up_next: PlayerColor,
}

// Public methods
impl Board {
    /// Check if the board is possible to achieve in gameplay.
    pub fn is_valid(&self) -> bool {
        let no_overlap = (self.yellow | self.red) == 0;

        no_overlap && self.validate_checker_count() && !(self.yellow & self.red).is_hanging()
    }

    /// Iterate over all the legal moves from this board state
    pub fn iter_moves(&self) -> impl Iterator<Item = Move> {
        (0..NUM_COLUMNS)
            .filter(|col_idx| {
                // Make sure the top cell of the column isn't occupied
                !self.combined_bitboard().is_top_cell_filled(*col_idx)
            })
            .map(|col_idx| Move::new(col_idx as u8))
    }

    pub fn apply_move(mut self, r#move: Move) -> Self {
        let col_idx: usize = r#move.column() as usize;
        assert!(
            !self.combined_bitboard().is_top_cell_filled(col_idx),
            "Tried to move in an already filled column"
        );
        debug_assert!(self.is_valid(), "Tried to apply a move to an invalid board");

        // The number of checkers stacked in the column
        let col_stack_height: usize = (self.combined_bitboard()
            & Bitboard::NTH_COLUMN[r#move.column() as usize])
            .count_ones() as usize;

        let mask = Bitboard::NTH_ROW[col_stack_height] & Bitboard::NTH_COLUMN[col_idx];
        match self.up_next {
            PlayerColor::Yellow => self.yellow |= mask,
            PlayerColor::Red => self.red |= mask,
        }
        self.up_next = self.up_next.next();
        self
    }

    /// Validate that the numbers of red and yellow checkers are legal
    fn validate_checker_count(&self) -> bool {
        let checker_difference = self.yellow.count_ones() - self.red.count_ones();
        match checker_difference {
            0 => self.up_next == PlayerColor::Yellow,
            1 => self.up_next == PlayerColor::Red,
            _ => return false,
        }
    }

    /// The union of the yellow and red bitboards
    fn combined_bitboard(&self) -> Bitboard {
        self.yellow | self.red
    }
}
