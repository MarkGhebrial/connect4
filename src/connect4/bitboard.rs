use std::ops::{BitAnd, BitOr};

use crate::connect4::bitboard::masks::NTH_ROW;

/// Bitboards are represented as follows:
/// ```text
/// 0  1  2  3  4  5  6
/// 7  8  9  10 11 12 13
/// 14 15 16 17 18 19 20
/// 21 22 23 24 25 26 27
/// 28 29 30 31 32 33 34
/// 35 35 37 38 39 40 41
/// // NOT THIS ONE ^^^^
/// // THIS ONE vvvvvv
/// 41 40 39 38 37 36 35
/// 34 33 32 31 30 29 28
/// 27 26 25 24 23 22 21
/// 20 19 18 17 16 15 14
/// 13 12 11 10 9  8  7
/// 6  5  4  3  2  1  0
/// ```
/// where 0 is the least significant bit. Unlike chess, we don't have a perfect 8x8 grid, so every
/// bitboard has 22 wasted bits at the end. Those bits must ALWAYS be zero. TODO for future Mark:
/// replace this type alias with a wrapper struct that
pub type Bitboard = u64;

pub const NUM_ROWS: usize = 6;
pub const NUM_COLUMNS: usize = 7;

pub mod masks {
    use crate::connect4::bitboard::{NUM_COLUMNS, NUM_ROWS};

    use super::Bitboard;

    /// A bitboard with no cells filled.
    pub const EMPTY: Bitboard = 0;
    /// A bitboard with every cell filled
    pub const FULL: Bitboard = 0x3FFFFFFFFFF;
    pub const BOTTOM_ROW: Bitboard = 0b1111111;
    pub const TOP_ROW: Bitboard = NTH_ROW[NUM_ROWS - 1];
    pub const RIGHTMOST_COLUMN: Bitboard = 0b0000001_0000001_0000001_0000001_0000001_0000001;

    /// Bitboard masks for each row of the table. The bottom row is number zero.
    pub const NTH_ROW: [Bitboard; NUM_ROWS] = {
        let mut rows = [EMPTY; NUM_ROWS];

        let mut i = 0;
        while i < NUM_ROWS {
            rows[i] = BOTTOM_ROW << (i * NUM_COLUMNS);
            i += 1;
        }

        rows
    };

    /// Bitboard masks for each column of the table. The rightmost column is number zero.
    pub const NTH_COLUMN: [Bitboard; NUM_COLUMNS] = {
        let mut columns = [EMPTY; NUM_COLUMNS];

        let mut i = 0;
        while i < NUM_COLUMNS {
            columns[i] = RIGHTMOST_COLUMN << i;
            i += 1;
        }

        columns
    };
}

/// Are any of the checkers in the bitboard defying gravity?
pub fn is_hanging(bb: Bitboard) -> bool {
    let mut is_hanging = false;
    for i in 1..NUM_ROWS {
        let current_row = (bb & NTH_ROW[i]) >> NUM_COLUMNS;
        let prev_row = bb & NTH_ROW[i - 1];
        is_hanging |= (current_row & prev_row).count_ones() < current_row.count_ones();
    }
    is_hanging
}

/// Create a bitboard from an array of bytes. Each byte represents one cell.
///
/// TODO: Consider making this function `const`, or making it a macro.
pub fn array_to_bitboard(array: [[u8; NUM_COLUMNS]; NUM_ROWS]) -> Bitboard {
    let mut bb: Bitboard = 0;
    for (i, byte) in array.iter().flatten().rev().enumerate() {
        bb |= ((*byte != 0) as u64) << i;
    }
    bb
}

// pub struct Bitboardd(u64);

// impl Bitboardd {
//     /// A bitboard with no cells filled.
//     pub const EMPTY: Bitboardd = Self::new(0);
//     /// A bitboard with every cell filled
//     pub const FULL: Bitboardd = Self::new(0x3FFFFFFFFFF);
//     pub const BOTTOM_ROW: Bitboardd = Self::new(0b1111111);

//     /// Create a new Bitboard. Clears the topmost (unused) bits
//     pub const fn new(n: u64) -> Self {
//         Self(n & FULL)
//     }
// }

// impl BitAnd for Bitboardd {
//     type Output = Self;
//     fn bitand(self, rhs: Self) -> Self::Output {
//         Self(self.0 & rhs.0)
//     }
// }

// impl BitOr for Bitboardd {
//     type Output = Self;
//     fn bitor(self, rhs: Self) -> Self::Output {
//         todo!()
//     }
// }
