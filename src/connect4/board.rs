use crate::connect4::{
    bitboard::{
        Bitboard, NUM_COLUMNS, is_hanging,
        masks::{NTH_COLUMN, TOP_ROW},
    },
    r#move::Move,
    player_color::PlayerColor,
};

/// A possible state of the board. The positions of the yellow and red checkers are represented as a
/// bitboards
/// where 0 is the least significant bit. Unlike chess, we don't have a perfect 8x8 grid, so every
/// bitboard has 22 wasted bits at the end.
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

        no_overlap && self.validate_checker_count() && !is_hanging(self.yellow & self.red)
    }

    /// Iterate over all the legal moves from this board state
    pub fn iter_moves(&self) -> impl Iterator<Item = Move> {
        (0..NUM_COLUMNS)
            .filter(|col_idx| {
                let combined_bb = self.yellow | self.red;
                (combined_bb & (TOP_ROW & NTH_COLUMN[*col_idx])) == 0
            })
            .map(|col_idx| Move::new(col_idx as u8))
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
}
