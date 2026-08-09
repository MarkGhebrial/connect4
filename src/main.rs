mod algo;
mod connect4;

use std::io::{stdin, stdout};

use clap::Parser;
use crossterm::{cursor, execute, style::Print, terminal};

use crate::{
    algo::search_moves, connect4::{bitboard::NUM_ROWS, board::Board, r#move::Move, player_color::PlayerColor},
};

/// Program for playing connect 4
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    subcommand: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Play an interactive game of connect 4 against the computer.
    Interactive(InteractiveArgs),
    /// Launch in "connect 4 interface" mode.
    C4I,
}

#[derive(clap::Parser, Debug)]
struct InteractiveArgs {
    #[arg(short, long, default_value = "yellow")]
    play_as: PlayerColor,
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Interactive(args) => run_interactive(args),
            Commands::C4I => run_c4i(),
        }
    }
}

fn main() {
    let args = Args::parse();

    args.subcommand.run();
}

fn run_interactive(args: &InteractiveArgs) {
    let mut board = Board::new();

    // Make space to draw the board
    execute!(
        stdout(),
        crossterm::terminal::ScrollUp(NUM_ROWS as u16 + 1),
        cursor::MoveUp(NUM_ROWS as u16 + 1),
        cursor::SavePosition,
    )
    .unwrap();

    while board.has_four_in_a_row().is_none() {
        // Display the board
        execute!(stdout(), cursor::RestorePosition, Print(board)).unwrap();

        // Human's turn
        if board.up_next() == args.play_as {
            let legal_moves: Vec<u8> = board.iter_moves().map(|m| m.column()).collect();

            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::CurrentLine),
                Print("Enter your move: ")
            )
            .unwrap();

            let move_: u8 = loop {
                let mut line = String::new();
                stdin().read_line(&mut line).unwrap();
                execute!(stdout(), terminal::ScrollDown(1),).unwrap();

                // Only accept inputs that are a single character
                let first_byte = if line.trim().len() == 1 {
                    line.bytes().nth(0)
                } else {
                    None
                };

                if let Some(byte) = first_byte
                    && legal_moves.contains(&byte.wrapping_sub(b'0'))
                {
                    break byte - b'0';
                } else {
                    execute!(
                        stdout(),
                        terminal::Clear(terminal::ClearType::CurrentLine),
                        Print("Invalid move. Try again: ")
                    )
                    .unwrap();
                }
            };

            board.apply_move(Move::new(move_));
        }
        // Computer's turn
        else {
            execute!(
                stdout(),
                terminal::Clear(terminal::ClearType::CurrentLine),
                Print("Computing next move...")
            )
            .unwrap();
            let (move_, _evaluation) = search_moves(&board, 1);
            board.apply_move(move_);
        }
    }

    // Display the board
    execute!(stdout(), cursor::RestorePosition, Print(board)).unwrap();

    let message = match board.has_four_in_a_row().unwrap() {
        PlayerColor::Yellow => "Yellow won!",
        PlayerColor::Red => "Red won!",
    };
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::CurrentLine),
        Print(message),
        Print("\n"),
    ).unwrap();
}

fn run_c4i() {
    todo!()
}
