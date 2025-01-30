//! Module to play with an UCI engine
//!
//! Some ressources
//! - Example of a typical workflow: https://stackoverflow.com/questions/17003561/using-the-universal-chess-interface
//! - Requirements of the UCI protocol: https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf


use std::io::{self, BufRead};
use vampirc_uci::{parse, MessageList, UciMessage, UciTimeControl};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock(); // Locking stdin for efficient reading

    for line in handle.lines() {
        match line {
            Ok(input) => {
                let messages: MessageList = parse(&input);
                
                for m in messages {
                    match m {
                        UciMessage::Uci => {
                            // Initialize the UCI mode of the chess engine.
                            println!("Setting up UCI engine mode.")
                        }
                        UciMessage::IsReady => {
                            // Engine is ready
                            // TODO answer `readyok`
                        }
                        UciMessage::UciNewGame => {
                            // Starting a new game
                        }
                        UciMessage::Position { startpos, fen, moves } => {
                        }
                        UciMessage::Go { time_control, .. } => {
                            // Set the depth
                        }
                        _ => println!("Unknown message: {:?}", m)
                    }
                }


            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }

}
