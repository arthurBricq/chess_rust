//! Module to play with an UCI engine
//!
//! Some ressources
//! - Example of a typical workflow: https://stackoverflow.com/questions/17003561/using-the-universal-chess-interface
//! - Requirements of the UCI protocol: https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf

mod uci_player;
mod uci_answers;

use std::fs::OpenOptions;
use std::io::{self, BufRead};
use vampirc_uci::{parse, MessageList, UciMessage, UciTimeControl};

use std::io::Write;
use crate::uci_player::UciPlayer;

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

    let mut uci_player = UciPlayer::new();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                if let Err(e) = writeln!(input_file, "{}", input) {
                    eprintln!("Error writing to inputs.txt: {}", e);
                }

                // Process the input as a UCI message
                let messages: MessageList = parse(&input);
                for m in messages {
                    let answer = uci_player.handle_message(m);
                    if let Some(output) = answer.into_formatted() {
                        // Print and save the output
                        println!("{}", output); // Ensure the output is printed

                        if let Err(e) = writeln!(output_file, "{}", output) {
                            eprintln!("Error writing to output.txt: {}", e);
                        }
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