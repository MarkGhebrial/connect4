use std::io::{self, stdin, stdout};

use c4board::{bitboard::NUM_ROWS, board::Board};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
};

pub struct Tui {
    cursor_home: (u16, u16),
    message_width: u16,
}

impl Tui {
    const TUI_HEIGHT: u16 = NUM_ROWS as u16 + 2;

    pub fn init() -> io::Result<Self> {
        // Grab the current position of the cursor
        let (_cursor_x, mut cursor_y) = crossterm::cursor::position()?;
        // Grab the current size of the terminal
        let (_term_width, term_height) = crossterm::terminal::size()?;

        // Make space for the TUI if needed
        let available_space = term_height - cursor_y;
        if available_space < Self::TUI_HEIGHT {
            execute!(
                stdout(),
                crossterm::terminal::ScrollUp(Self::TUI_HEIGHT - available_space)
            )?;
            cursor_y -= Self::TUI_HEIGHT - available_space;
        }

        Ok(Self {
            cursor_home: (0, cursor_y),
            message_width: 0,
        })
    }

    pub fn set_board(&self, board: &Board) -> io::Result<()> {
        execute!(
            stdout(),
            MoveTo(self.cursor_home.0, self.cursor_home.1),
            Print(board),
        )
    }

    /// Display the given message on the line beneath the board
    pub fn set_message(&mut self, message: &str) -> io::Result<()> {
        self.message_width = message.len() as u16;
        execute!(
            stdout(),
            MoveTo(
                self.cursor_home.0,
                self.cursor_home.1 + Self::TUI_HEIGHT - 1
            ),
            Clear(ClearType::CurrentLine),
            Print(message),
        )
    }

    pub fn get_input(&self, buf: &mut String) -> io::Result<()> {
        execute!(
            stdout(),
            crossterm::cursor::MoveTo(
                // self.cursor_home.0
                self.message_width,
                self.cursor_home.1 + Self::TUI_HEIGHT - 1,
            )
        )?;
        let needs_scroll = crossterm::cursor::position()?.1 == crossterm::terminal::size()?.1 - 1;
        stdin().read_line(buf)?;
        if needs_scroll {
            execute!(stdout(), crossterm::terminal::ScrollDown(1),)?;
        }
        Ok(())
    }
}
