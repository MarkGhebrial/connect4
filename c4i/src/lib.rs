use std::io::{self, BufRead, BufReader};

use c4board::{board::Board, r#move::Move};

/// c4i stands for "connect 4 interface". It's a protocol I invented that's vaguely inspired by the
/// universal chess interface. A typical exchange goes like this (client goes first):
/// ```text
/// c4i
/// c4iok
/// play ;;;;;;
/// playok 3
/// play ;;;ry;;;
/// playok 4
/// ```
/// Those chunks of text are boards serialized using my "connect 4 encoding" (c4e) format. See
/// [Board::from_c4e] for more detail.
pub trait C4IServer {
    fn c4i() -> String {
        String::from("c4iok")
    }

    fn play(board: &Board) -> Move;

    fn start_server<R, W>(reader: &mut R, writer: &mut W) -> io::Result<()>
    where
        R: io::Read,
        W: io::Write,
    {
        let mut line = String::new();

        let mut reader = BufReader::new(reader);

        loop {
            line.clear();
            reader.read_line(&mut line)?;

            let mut words = line.split_whitespace();
            match words.next() {
                Some("c4i") => writeln!(writer, "{}", Self::c4i())?,
                Some("play") => {
                    let Some(c4e) = words.next() else {
                        writeln!(writer, "err missing argument")?;
                        continue;
                    };

                    let board = match Board::from_c4e(c4e) {
                        Ok(board) => board,
                        Err(e) => {
                            writeln!(writer, "err {}", e)?;
                            continue;
                        }
                    };

                    if board.is_game_over() {
                        writeln!(writer, "err no legal moves")?;
                        continue;
                    }

                    writeln!(writer, "playok")?;
                    let move_ = Self::play(&board);
                    writeln!(writer, "playdone {}", move_.column())?;
                }
                Some(_) => writeln!(writer, "err unrecognized command")?,
                None => (),
            }
        }
    }
}

// pub struct C4IClient<'a, R, W>
// where
//     R: io::BufRead,
//     W: io::Write,
// {
//     reader: &'a R,
//     writer: &'a W,
// }
// impl C4IClient

pub mod client {
    use c4board::{board::Board, r#move::Move};
    use std::io;

    pub fn greet<RW>(rw: &mut RW) -> bool
    where
        RW: io::BufRead + io::Write,
    {
        if writeln!(rw, "c4i").is_err() {
            return false;
        };
        let mut line = String::new();
        if rw.read_line(&mut line).is_err() {
            return false;
        }
        let Some(first_word) = line.split_whitespace().nth(0) else {
            return false;
        };
        if first_word != "c4iok" {
            return false;
        }

        true
    }

    pub fn play<RW>(rw: &mut RW, board: &Board) -> io::Result<Move>
    where
        RW: io::BufRead + io::Write,
    {
        writeln!(rw, "play {}", board.to_c4e())?;

        todo!();
    }
}
