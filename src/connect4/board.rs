use std::{debug_assert, fmt::Display};

use crossterm::style::Stylize;

use crate::connect4::{
    bitboard::{Bitboard, NUM_COLUMNS, NUM_ROWS},
    err::C4eParseError,
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
    pub fn new() -> Self {
        Self {
            yellow: Bitboard::EMPTY,
            red: Bitboard::EMPTY,
            up_next: PlayerColor::Yellow,
        }
    }

    /// Parse a c4e string into a board.
    pub fn from_c4e(c4e: &str) -> Result<Self, C4eParseError> {
        let columns: Vec<&str> = c4e.split(';').collect();

        if columns.len() != NUM_COLUMNS {
            return Err(C4eParseError::WrongNumberOfColumns);
        }

        let mut board = Self::new();
        for (column_index, column) in columns.iter().rev().enumerate() {
            if column.len() > NUM_ROWS {
                return Err(C4eParseError::OverfilledColumn);
            }

            for byte in column.bytes() {
                board.drop_into_column(
                    column_index,
                    match byte {
                        b'y' => PlayerColor::Yellow,
                        b'r' => PlayerColor::Red,
                        _ => return Err(C4eParseError::IllegalCharacter),
                    },
                );
            }
        }
        let checker_difference = board
            .yellow
            .count_ones()
            .wrapping_sub(board.red.count_ones());
        board.up_next = if checker_difference % 2 == 0 {
            PlayerColor::Yellow
        } else {
            PlayerColor::Red
        };

        if !board.is_valid() {
            return Err(C4eParseError::InvalidBoard);
        }

        Ok(board)
    }

    pub fn yellow(&self) -> Bitboard {
        self.yellow
    }

    pub fn red(&self) -> Bitboard {
        self.yellow
    }

    /// Who's turn is it?
    pub fn up_next(&self) -> PlayerColor {
        self.up_next
    }

    /// Check if the board is possible to achieve in gameplay.
    pub fn is_valid(&self) -> bool {
        let no_overlap = (self.yellow & self.red) == 0;

        no_overlap && self.validate_checker_count() && !self.combined_bitboard().is_hanging()
    }

    pub fn has_four_in_a_row(&self) -> Option<PlayerColor> {
        let yellow_four = self.yellow.has_four_in_a_row();
        let red_four = self.red.has_four_in_a_row();
        match (yellow_four, red_four) {
            (true, true) => panic!("Both players have four in a row."),
            (true, false) => Some(PlayerColor::Yellow),
            (false, true) => Some(PlayerColor::Red),
            (false, false) => None,
        }
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

    pub fn apply_move(&mut self, r#move: Move) {
        debug_assert!(self.is_valid(), "Tried to apply a move to an invalid board");
        self.drop_into_column(r#move.column() as usize, self.up_next);
        self.up_next = self.up_next.next();
    }

    pub fn with_move(mut self, r#move: Move) -> Self {
        self.apply_move(r#move);
        self
    }

    /// Drop a checker into the specified column of the board. Note that this function offers no
    /// protection against creating an invalid board state, and it doesn't advance `self.up_next()`
    fn drop_into_column(&mut self, col_idx: usize, color: PlayerColor) {
        assert!(
            !self.combined_bitboard().is_top_cell_filled(col_idx),
            "Tried to move in an already filled column"
        );

        // The number of checkers stacked in the column
        let col_stack_height: usize =
            (self.combined_bitboard() & Bitboard::NTH_COLUMN[col_idx]).count_ones() as usize;

        let mask = Bitboard::NTH_ROW[col_stack_height] & Bitboard::NTH_COLUMN[col_idx];
        match color {
            PlayerColor::Yellow => self.yellow |= mask,
            PlayerColor::Red => self.red |= mask,
        }
    }

    pub fn pretty_print(
        &self,
        mut writer: impl std::fmt::Write,
        with_numbers: bool,
    ) -> std::fmt::Result {
        for i in (0..(NUM_ROWS * NUM_COLUMNS)).rev() {
            let col_idx = i % NUM_COLUMNS;

            if col_idx == NUM_COLUMNS - 1 {
                write!(writer, "┃ ")?;
            }

            let cell_mask = Bitboard::new(1 << i);

            let is_yellow = (self.yellow & cell_mask) != 0;
            let is_red = (self.red & cell_mask) != 0;
            match (is_yellow, is_red) {
                (true, true) => panic!("Tried to pretty print board with overlapping checkers"),
                (true, false) => write!(writer, "{} ", "x".yellow())?,
                (false, true) => write!(writer, "{} ", "o".red())?,
                (false, false) => write!(writer, ". ")?,
            }

            if col_idx == 0 {
                writeln!(writer, "┃")?;
            }
        }
        if with_numbers {
            // TODO: Use this labeling instead: println!("┗━1━2━3━4━5━6━7━┛")
            writeln!(writer, "┗━6━5━4━3━2━1━0━┛")
        } else {
            writeln!(writer, "┗━━━━━━━━━━━━━━━┛")
        }
    }

    /// Validate that the numbers of red and yellow checkers are legal
    fn validate_checker_count(&self) -> bool {
        let checker_difference = self.yellow.count_ones().wrapping_sub(self.red.count_ones());
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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, true)
    }
}
