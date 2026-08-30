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

pub struct C4IClient<R, W>
where
    R: io::Read,
    W: io::Write,
{
    reader: BufReader<R>,
    writer: W,
}

impl<R, W> C4IClient<R, W>
where
    R: io::Read,
    W: io::Write,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader: BufReader::new(reader),
            writer,
        }
    }

    pub fn greet(&mut self) -> io::Result<()> {
        writeln!(self.writer, "c4i")?;
        let mut line = String::new();
        // Read lines until we get one that's not empty
        while line.is_empty() || line == "\n" {
            line.clear();
            self.reader.read_line(&mut line)?;
        }
        // Verify that the response is "c4iok\n". TODO: Handle this more gracefully
        assert_eq!(line, "c4iok\n");
        Ok(())
    }

    pub fn play(&mut self, board: &Board) -> io::Result<Move> {
        writeln!(self.writer, "play {}", board.to_c4e())?;
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        // TODO: Do literally any error handling at all
        assert_eq!(line, "playok\n");

        line.clear();
        self.reader.read_line(&mut line)?;
        let mut words = line.split_whitespace();
        // TODO: Do literally any error handling at all
        assert_eq!(words.next().unwrap(), "playdone");
        let move_ = Move::new(words.next().unwrap().parse().unwrap());

        Ok(move_)
    }
}
