mod tui;

use std::io;

use clap::Parser;

use c4board::{board::Board, r#move::Move, player_color::PlayerColor};
use c4engine::search_moves;

use crate::tui::Tui;

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
    Play(PlayArgs),
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Play(args) => run_interactive(args).unwrap(),
        }
    }
}

#[derive(clap::Parser, Debug)]
struct PlayArgs {
    #[arg(short, long, default_value = "yellow")]
    play_as: PlayAs,
}

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
enum PlayAs {
    Yellow,
    Red,
}
impl PlayAs {
    pub fn to_player_color(&self) -> PlayerColor {
        match self {
            PlayAs::Yellow => PlayerColor::Yellow,
            PlayAs::Red => PlayerColor::Red,
        }
    }
}
impl PartialEq<PlayAs> for PlayerColor {
    fn eq(&self, other: &PlayAs) -> bool {
        self == &other.to_player_color()
    }
}

fn run_interactive(args: &PlayArgs) -> io::Result<()> {
    let mut board = Board::new();

    let mut tui = Tui::init()?;

    while !board.is_game_over() {
        // Display the board
        tui.set_board(&board)?;

        // Human's turn
        if board.up_next() == args.play_as {
            let legal_moves: Vec<u8> = board.iter_moves().map(|m| m.column()).collect();

            let mut line = String::new();
            tui.set_message("Enter your move: ")?;

            let move_: u8 = loop {
                line.clear();
                tui.get_input(&mut line)?;

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
                    tui.set_message("Invalid move. Try again: ")?;
                }
            };

            board.apply_move(Move::new(move_));
        }
        // Computer's turn
        else {
            tui.set_message("Computing next move...")?;
            let (move_, _evaluation) = search_moves(&board, 4).unwrap();
            board.apply_move(move_);
        }
    }

    tui.set_board(&board)?;

    let message = match board.has_four_in_a_row().unwrap() {
        PlayerColor::Yellow => "Yellow won!",
        PlayerColor::Red => "Red won!",
    };
    tui.set_message(message)
}

fn main() {
    let args = Args::parse();

    args.subcommand.run();
}
