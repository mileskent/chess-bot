use chess::{Board, MoveGen};
use rand::seq::IteratorRandom;
use std::io::{self, BufRead, Write};
use std::str::FromStr;
use vampirc_uci::{parse_one, UciMessage};

fn main() {
    let mut board = Board::default();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    let botname = String::new("chess-bot");

    loop {
        line.clear();
        if (handle.read_line(&mut line).unwrap() == 0) { break; }

        let message = parse_one(&line);
        match message {
            // Identify
            UciMessage::Uci => {
                println!(f"id name {botname}");
                println!("id author MilesKent");
                println!("uciok");
            }

            // Acknowledge ready
            UciMessage::IsReady => {
                println!("readyok");
            }

            // Reset state to new game
            UciMessage::UciNewGame => {
                board = Board::default();
            }

            // Sync state with GUI
            UciMessage::Position { startpos, fen, moves } => {
                if startpos {
                    board = Board::default();
                } else if let Some(f) = fen {
                    if let Ok(b) = Board::from_str(&f.to_string()) {
                        board = b;
                    }
                }

                for m in moves {
                    board = board.make_move_new(m);
                }
            }

            // Start thinking for move...
            UciMessage::Go { .. } => {
                let move_gen = MoveGen::new_legal(&board);

                let mut rng = rand::thread_rng();
                if let Some(random_move) = move_gen.choose(&mut rng) {
                    println!("bestmove {}", random_move);
                }
            }

            // Quit
            UciMessage::Quit => break,

            // Ignore everything else
            _ => {}
        }

        // Flush stdout for speed
        io::stdout().flush().unwrap();
    }
}
