//! Module to play with an UCI engine
//!
//! Some ressources
//! - Example of a typical workflow: https://stackoverflow.com/questions/17003561/using-the-universal-chess-interface
//! - Requirements of the UCI protocol: https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf

mod uci_player;
mod uci_answers;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use vampirc_uci::{parse, MessageList, UciMessage, UciTimeControl};

use std::io::Write;
use crate::uci_player::UciPlayer;

fn write_to_file(mut file: &File, content: &str, prefix: &str)  {
    if let Err(e) = writeln!(file, "{prefix:<7} >>> {}", content) {
        eprintln!("Error writing to output.txt: {}", e);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true) // Enable write mode (and disable appending)
        .truncate(true) // Ensure the file is cleared if it already exists
        .open("output.txt")
        .expect("Failed to open output.txt");

    let mut uci_player = UciPlayer::new();

    for line in handle.lines() {
        match line {
            Ok(input) => {
                write_to_file(&mut output_file, &input, "INPUT");

                // Process the input as a UCI message
                let messages: MessageList = parse(&input);
                for m in messages {
                    let answers = uci_player.handle_message(m).into_formatted();

                    match answers {
                        (None, Some(msg)) => write_to_file(&mut output_file, &msg, "DEBUG"),
                        (Some(msg), _ ) => {
                            // Print and save the output
                            println!("{}", msg); // Ensure the output is printed
                            write_to_file(&mut output_file, &msg, "OUTPUT")
                        }
                        (None, None) => {}
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