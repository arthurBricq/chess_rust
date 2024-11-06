use crate::model::chess_type::Type;
use crate::model::game::ChessGame;
use crate::model::moves::Move;
use crate::model::tools::pos_to_index;
use regex::Regex;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

pub struct TerminalChessView<'a> {
    game: &'a mut ChessGame,
}

impl<'a> TerminalChessView<'a> {
    pub fn new(_game: &'a mut ChessGame) -> Self {
        Self {
            game: _game
        }
    }

    pub fn get_char_at(&self, i: i8, j: i8) -> String {
        if let Some(t) = self.game.type_at(i, j) {
            if self.game.is_white_at(i, j) {
                match t {
                    Type::Pawn => { "♙".to_string() }
                    Type::Bishop => { "♗".to_string() }
                    Type::Knight => { "♘".to_string() }
                    Type::Rook => { "♖".to_string() }
                    Type::Queen => { "♕".to_string() }
                    Type::King => { "♔".to_string() }
                }
            } else {
                match t {
                    Type::Pawn => { "♙".to_string() }
                    Type::Bishop => { "♝".to_string() }
                    Type::Knight => { "♞".to_string() }
                    Type::Rook => { "♜".to_string() }
                    Type::Queen => { "♛".to_string() }
                    Type::King => { "♚".to_string() }
                }
            }
        } else {
            " ".to_string()
        }
    }

    pub fn display(&'a self) {
        print!("\n    |---------------------------------------| \n");
        for i in (0..8).rev() {
            print!("    | ");
            for j in 0..8 {
                let s = self.get_char_at(j, i);
                print!("{s}");
                print!(" | ")
            }
            // print!("\n    |-------------------------------|");
            print!("\n");
        }
        print!("\n    |---------------------------------------| \n");
    }
}


impl<'a> TerminalChessView<'a> {
    pub fn play(&mut self) {
        // Read the input
        let mut s = String::new();
        println!("Enter move to play: ");
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a valid string");
        println!("You typed: {s}");

        // Parse the input
        let re = Regex::new(r"(\d),(\d) to (\d),(\d)").unwrap();
        if let Some(cap) = re.captures(&s) {
            if cap.len() == 5 {
                let x1 = i8::from_str(&cap[1]).unwrap();
                let y1 = i8::from_str(&cap[2]).unwrap();
                let x2 = i8::from_str(&cap[3]).unwrap();
                let y2 = i8::from_str(&cap[4]).unwrap();

                let m = Move::new(pos_to_index(x1, y1), pos_to_index(x2, y2), true);

                let success = self.game.apply_move_safe(
                    m
                );

                println!("Success of the move: {success}");

                self.display();
            }
        }
    }
}