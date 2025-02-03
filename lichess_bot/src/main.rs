//! Module to play with an UCI engine
//!
//! Some ressources
//! - Example of a typical workflow: https://stackoverflow.com/questions/17003561/using-the-universal-chess-interface
//! - Requirements of the UCI protocol: https://gist.github.com/DOBRO/2592c6dad754ba67e6dcaec8c90165bf

mod uci_answers;
mod uci_player;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead};
use vampirc_uci::{parse, MessageList};

use crate::uci_player::UciPlayer;
use std::io::Write;

fn write_to_file(mut file: &File, content: &str, prefix: &str) {
    if let Err(e) = writeln!(file, "{prefix:<7} >>> {}", content) {
        eprintln!("Error writing to output.txt: {}", e);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut output_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Failed to open output.txt");

    // Write the header
    let current_time = chrono::Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S");
    if let Err(e) = writeln!(output_file, "\n###### NEW PROCESS: {}", formatted_time) {
        eprintln!("Error writing to output.txt: {}", e);
    }

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
                        (Some(msg), _) => {
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
