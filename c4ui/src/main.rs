mod tui;

use std::{io, num::NonZeroUsize, process::Stdio};

use clap::Parser;

use c4board::{bitboard::NUM_COLUMNS, board::Board, r#move::Move, player_color::PlayerColor};
use c4engine::search_moves;
use rand::random_range;

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
    /// Run a tournament between two engines.
    Tourney(TourneyArgs),
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Play(args) => run_interactive(args).unwrap(),
            Commands::Tourney(args) => run_tourney(args).unwrap(),
        }
    }
}

#[derive(clap::Parser, Debug)]
struct PlayArgs {
    #[arg(short, long, default_value = "yellow")]
    play_as: PlayAs,

    against: Option<Vec<String>>,
}

#[derive(clap::Parser, Debug)]
struct TourneyArgs {
    /// The command to use to invoke the first engine. If not specified, uses the engine built into
    /// c4ui.
    engine_a_command: Option<String>,
    /// The command to use to invoke the second engine. If not specified, uses the engine built into
    /// c4ui.
    engine_b_command: Option<String>,

    /// How many games to play
    #[arg(short, long, default_value = "300")]
    num_games: NonZeroUsize,
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
            let (move_, _evaluation) = search_moves(&board, 11).unwrap();
            board.apply_move(move_);
        }
    }

    tui.set_board(&board)?;

    let message = match board.winner() {
        Some(PlayerColor::Yellow) => "Yellow won!",
        Some(PlayerColor::Red) => "Red won!",
        None => "Draw.",
    };
    tui.set_message(message)
}

struct C4IProcess {
    child_process: std::process::Child,
    client: c4i::C4IClient<std::process::ChildStdout, std::process::ChildStdin>,
}

impl C4IProcess {
    pub fn new(command: &str) -> io::Result<Self> {
        let mut child = std::process::Command::new(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let stdin = child
            .stdin
            .take()
            .expect("couldn't take child process' stdin");
        let stdout = child
            .stdout
            .take()
            .expect("couldn't take child process' stdout");
        let mut client = c4i::C4IClient::new(stdout, stdin);

        client.greet()?;

        Ok(Self {
            child_process: child,
            client,
        })
    }

    pub fn play(&mut self, board: &Board) -> io::Result<Move> {
        self.client.play(board)
    }
}

impl Drop for C4IProcess {
    fn drop(&mut self) {
        self.child_process
            .kill()
            .expect("unable to kill child process");
    }
}

enum Player {
    // The version of c4engine that's bundled with c4ui
    Static,
    // An external, c4i compatible, connect 4 engine
    C4i(C4IProcess),
}

impl Player {
    pub fn play(&mut self, board: &Board) -> Move {
        match self {
            Player::Static => c4engine::search_moves(board, 11).unwrap().0,
            Player::C4i(c4i_player) => c4i_player.play(board).unwrap(),
        }
    }
}

struct WinCounter {
    pub player_a_win_count: usize,
    pub player_b_win_count: usize,
    pub draw_count: usize,
}
impl WinCounter {
    pub fn new() -> Self {
        Self {
            player_a_win_count: 0,
            player_b_win_count: 0,
            draw_count: 0,
        }
    }
    fn total(&self) -> usize {
        self.player_a_win_count + self.player_b_win_count + self.draw_count
    }

    pub fn player_a_win_ratio(&self) -> f32 {
        self.player_a_win_count as f32 / self.total() as f32
    }
    pub fn player_b_win_ratio(&self) -> f32 {
        self.player_b_win_count as f32 / self.total() as f32
    }
    pub fn draw_ratio(&self) -> f32 {
        self.draw_count as f32 / self.total() as f32
    }
}

fn run_tourney(args: &TourneyArgs) -> io::Result<()> {
    // Load up the two opponents
    let mut player_a = match &args.engine_a_command {
        Some(command) => Player::C4i(C4IProcess::new(&command)?),
        None => Player::Static,
    };
    let mut player_b = match &args.engine_b_command {
        Some(command) => Player::C4i(C4IProcess::new(&command)?),
        None => Player::Static,
    };

    let mut counter = WinCounter::new();

    let mut tui = Tui::init()?;
    for _game_number in 0..args.num_games.get() {
        let mut board = Board::new();

        // Take six random moves
        for _ in 0..6 {
            let col: u8 = random_range(0..NUM_COLUMNS as u8);
            let move_ = Move::new(col);
            board.apply_move(move_);
        }

        tui.set_board(&board)?;

        // Play a game where player a is yellow
        let game_a_winner = play_tourney_game(&mut tui, board, &mut player_a, &mut player_b)?;
        let message = match game_a_winner {
            Some(PlayerColor::Yellow) => {
                counter.player_a_win_count += 1;
                "Player A won!"
            }
            Some(PlayerColor::Red) => {
                counter.player_b_win_count += 1;
                "Player B won!"
            }
            None => {
                counter.draw_count += 1;
                "Draw."
            }
        };
        tui.set_message(message)?;

        // Play a game where player b is yellow
        let game_b_winner = play_tourney_game(&mut tui, board, &mut player_b, &mut player_a)?;
        let message = match game_b_winner {
            Some(PlayerColor::Yellow) => {
                counter.player_b_win_count += 1;
                "Player B won!"
            }
            Some(PlayerColor::Red) => {
                counter.player_a_win_count += 1;
                "Player A won!"
            }
            None => {
                counter.draw_count += 1;
                "Draw."
            }
        };
        tui.set_message(message)?;
    }

    tui.end()?;

    println!(
        "A D B: {} {} {}; {}% {}%, {}%",
        counter.player_a_win_count,
        counter.draw_count,
        counter.player_b_win_count,
        counter.player_a_win_ratio() * 100.0,
        counter.draw_ratio() * 100.0,
        counter.player_b_win_ratio() * 100.0,
    );

    Ok(())
}

fn play_tourney_game<'a>(
    tui: &mut Tui,
    mut board: Board,
    mut yellow_player: &'a mut Player,
    mut red_player: &'a mut Player,
) -> io::Result<Option<PlayerColor>> {
    tui.set_board(&board)?;

    while !board.is_game_over() {
        let (player, message) = match board.up_next() {
            PlayerColor::Yellow => (&mut yellow_player, "Yellow to play..."),
            PlayerColor::Red => (&mut red_player, "Red to play"),
        };

        tui.set_message(message)?;

        let move_ = player.play(&board);

        board.apply_move(move_);
        tui.set_board(&board)?;
    }

    Ok(board.winner())
}

fn main() {
    let args = Args::parse();

    args.subcommand.run();
}
