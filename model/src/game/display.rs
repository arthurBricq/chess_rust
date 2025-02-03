use crate::chess_type::Type;
use crate::game::ChessGame;

impl ChessGame {
    pub fn display(&self) {
        print!("");
        for i in (0..8).rev() {
            print!("    | ");
            for j in 0..8 {
                let s = self.get_char_at(j, i);
                print!("{s}");
                print!(" | ")
            }
            print!("\n");
        }
    }

    fn get_char_at(&self, i: i8, j: i8) -> String {
        if let Some(t) = self.type_at_xy(i, j) {
            if self.is_white_at_xy(i, j) {
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
                    Type::Pawn => { "♟".to_string() }
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
}
