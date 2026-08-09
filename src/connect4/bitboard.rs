use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, Shr},
};

/// The number of rows in a connect 4 grid
pub const NUM_ROWS: usize = 6;
/// The number of columns in a connect 4 grid
pub const NUM_COLUMNS: usize = 7;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl Bitboard {
    /// A bitboard with no cells filled.
    pub const EMPTY: Bitboard = Self(0);
    /// A bitboard with every cell filled
    pub const FULL: Bitboard = Self(0x3FFFFFFFFFF);
    pub const BOTTOM_ROW: Bitboard = Self(0b1111111);
    pub const TOP_ROW: Bitboard = Self::NTH_ROW[NUM_ROWS - 1];
    pub const RIGHTMOST_COLUMN: Bitboard = Self(0b0000001_0000001_0000001_0000001_0000001_0000001);

    /// Bitboard masks for each row of the table. The bottom row is number zero.
    pub const NTH_ROW: [Bitboard; NUM_ROWS] = {
        let mut rows = [Self::EMPTY; NUM_ROWS];

        let mut i = 0;
        while i < NUM_ROWS {
            // Grr I can't use the implementation of Bitboard::shl in const contexts...
            rows[i] = Self(Self::BOTTOM_ROW.0 << (i * NUM_COLUMNS));
            i += 1;
        }

        rows
    };

    /// Bitboard masks for each column of the table. The rightmost column is number zero.
    pub const NTH_COLUMN: [Bitboard; NUM_COLUMNS] = {
        let mut columns = [Self::EMPTY; NUM_COLUMNS];

        let mut i = 0;
        while i < NUM_COLUMNS {
            columns[i] = Self(Self::RIGHTMOST_COLUMN.0 << i);
            i += 1;
        }

        columns
    };

    /// Masks for the diagonals of the board that slope "up".
    pub const UP_DIAGONALS: [Bitboard; NUM_ROWS * 2] = {
        let mut base_diagonals: [Bitboard; NUM_ROWS] = [Self::FULL; NUM_ROWS];
        let base_diagonal = Self(0b0000010_0000100_0001000_0010000_0100000_1000001);
        
        let mut row = 0;
        while row < NUM_ROWS {
            base_diagonals[row] = Self(base_diagonal.0 << row);
            row += 1;
        }

        // Each one of the base diagonals "wraps" back around from the left to right side of the
        // board. We don't any wrapping, so we split the base diagonals at that split point.
        let mut split_diagonals: [Bitboard; NUM_ROWS * 2] = [Self::FULL; NUM_ROWS * 2];
        let mut row = 0;
        while row < NUM_ROWS {
            // Create the masks required to "split" the byteboard into the two diagonals
            let base_mask = 0b0111111;
            let mut a_mask: u64 = 0;
            let mut b_mask: u64 = 0;
            let mut i = 0;
            while i < NUM_COLUMNS {
                if i <= row {
                    a_mask |= base_mask << (i * NUM_ROWS);
                } else {
                    b_mask |= base_mask << (i * NUM_ROWS);
                }
                i += 1;
            }

            split_diagonals[row] = Self(base_diagonals[row].0 & a_mask);
            split_diagonals[row + NUM_ROWS] = Self(base_diagonals[row].0 & b_mask);

            row += 1;
        }
        
        split_diagonals
    };

    /// The set of all possible fours in a row.
    pub const FOURS: [Bitboard; 69] = {
        let mut fours = [Self::FULL; 69];
        let mut fours_index = 0;

        
        // Horizontal fours
        let mut row = 0;
        while row < NUM_ROWS {
            let mut i = 0;
            while i <= NUM_COLUMNS - 4 {
                fours[fours_index] = Self(Self::NTH_ROW[row].0 & (0b1111 << i << (row * NUM_COLUMNS)));
                fours_index += 1;
                i += 1;
            }
            row += 1;
        }

        // Vertical fours
        let mut column = 0;
        while column < NUM_COLUMNS {
            let mut i = 0;
            while i <= NUM_ROWS - 4 {
                fours[fours_index] = Self(Self::NTH_COLUMN[column].0 & (0b0000001_0000001_0000001_0000001 << column << (i * NUM_COLUMNS)));
                fours_index += 1;
                i += 1;
            }
            column += 1;
        }

        // Diagonal fours
        let base_up_four = Self::UP_DIAGONALS[3];
        // let base_down_four = Self::FULL; // TODO
        let base_down_four = Self(0b0000000_0000000_0001000_0000100_0000010_0000001);
        let mut col_shift = 0;
        while col_shift <= NUM_COLUMNS - 4 {
            let mut row_shift = 0;
            while row_shift <= NUM_ROWS - 4 {
                fours[fours_index] = Self(base_up_four.0 << col_shift << (NUM_COLUMNS * row_shift));
                fours[fours_index + 1] = Self(base_down_four.0 << col_shift << (NUM_COLUMNS * row_shift));
                fours_index += 2;
                row_shift += 1;
            }
            col_shift += 1;
        }

        fours
    };

    /// Create a new Bitboard. Clears the topmost (unused) bits
    pub const fn new(n: u64) -> Self {
        Self(n & Self::FULL.0)
    }

    /// Create a bitboard from an array of bytes. Each byte represents one cell.
    ///
    /// TODO: Consider making this function `const`, or making it a macro.
    #[allow(unused)] // This is currently only used in tests.
    pub fn from_array(array: [[u8; NUM_COLUMNS]; NUM_ROWS]) -> Bitboard {
        let mut bb: u64 = 0;
        for (i, byte) in array.iter().flatten().rev().enumerate() {
            bb |= ((*byte != 0) as u64) << i;
        }
        Self(bb)
    }

    /// Are any of the checkers in the bitboard defying gravity?
    pub fn is_hanging(&self) -> bool {
        let mut is_hanging = false;
        for i in 1..NUM_ROWS {
            let current_row = (*self & Self::NTH_ROW[i]) >> NUM_COLUMNS;
            let prev_row = *self & Self::NTH_ROW[i - 1];
            is_hanging |= (current_row & prev_row).count_ones() < current_row.count_ones();
        }
        is_hanging
    }

    pub fn has_four_in_a_row(&self) -> bool {
        let mut has_four_in_a_row = false;
        for mask in Self::FOURS {
            has_four_in_a_row |= *self & mask == mask;
        }
        has_four_in_a_row
    }

    /// Count the number of set bits.
    pub const fn count_ones(&self) -> u32 {
        self.0.count_ones()
    }

    /// Is the bit for the topmost cell of the given column set?
    ///
    /// TODO: This can probably be replaced with is_column_filled
    pub fn is_top_cell_filled(&self, column_index: usize) -> bool {
        (*self & (Self::TOP_ROW & Self::NTH_COLUMN[column_index])) != 0
    }

    #[allow(unused)]
    pub fn pretty_print(&self) {
        for i in (0..(NUM_ROWS * NUM_COLUMNS)).rev() {
            let col_idx = i % NUM_COLUMNS;

            if col_idx == NUM_COLUMNS - 1 {
                print!("┃ ");
            }

            let cell_mask = Bitboard::new(1 << i);

            if *self & cell_mask != 0 {
                print!( "x ");
            } else {
                print!( ". ");
            }

            if col_idx == 0 {
                println!("┃");
            }
        }
        println!("┗━6━5━4━3━2━1━0━┛");
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}
impl BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}
impl Shl<usize> for Bitboard {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        Self::new(self.0 << rhs)
    }
}
impl Shr<usize> for Bitboard {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        Self::new(self.0 >> rhs)
    }
}
impl PartialEq<u64> for Bitboard {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}
