//! Module to play with an UCI engine
//!
//! Some ressources
//! - Example of a typical workflow: https://stackoverflow.com/questions/17003561/using-the-universal-chess-interface
//! - Requirements of the UCI protocol: https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf

use std::fs::OpenOptions;
use std::io::{self, BufRead};
use vampirc_uci::{parse, MessageList, UciMessage, UciTimeControl};

use std::io::Write;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Open "inputs.txt" and "output.txt" in append mode
    let mut input_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("inputs.txt")
        .expect("Failed to open inputs.txt");
    
    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Failed to open output.txt");

    for line in handle.lines() {
        match line {
            Ok(input) => {
                // Print and save the input
                println!("{}", input); // Ensure the input is printed
                if let Err(e) = writeln!(input_file, "{}", input) {
                    eprintln!("Error writing to inputs.txt: {}", e);
                }

                // Process the input as a UCI message
                let messages: MessageList = parse(&input);
                for m in messages {
                    let output = match m {
                        UciMessage::Uci => {
                            "Setting up UCI engine mode.".to_string()
                        }
                        UciMessage::IsReady => {
                            "Engine is ready.".to_string()
                        }
                        UciMessage::UciNewGame => {
                            "Starting a new game.".to_string()
                        }
                        UciMessage::Position { startpos, fen, moves } => {
                            format!(
                                "Position received. Startpos: {:?}, FEN: {:?}, Moves: {:?}",
                                startpos, fen, moves
                            )
                        }
                        UciMessage::Go { time_control, .. } => {
                            match time_control {
                                Some(tc) => format!("Go command received with time control: {:?}", tc),
                                None => "Go command received with no time control.".to_string(),
                            }
                        }
                        _ => format!("Unknown message: {:?}", m),
                    };

                    // Print and save the output
                    println!("{}", output); // Ensure the output is printed
                    
                    if let Err(e) = writeln!(output_file, "{}", output) {
                        eprintln!("Error writing to output.txt: {}", e);
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