use crate::game::ChessGame;
use crate::utils::{pos_to_index, set_at, ChessPosition};

pub struct GameConstructor;

impl GameConstructor {
    /// An empty chess game
    #[allow(dead_code)]
    pub fn empty() -> ChessGame {
        ChessGame {
            whites: 0,
            pawns: 0,
            bishops: 0,
            knights: 0,
            rooks: 0,
            queens: 0,
            kings: 0,
            flags: 0,
        }
    }

    /// Constructor for a normal chess game.
    /// The pieces are set like on a normal chess set.
    pub fn standard_game() -> ChessGame {
        let mut whites = 0;
        let mut pawns = 0;
        let mut bishops = 0;
        let mut knights = 0;
        let mut rooks = 0;
        let mut queens = 0;
        let mut kings = 0;

        // Add the pawns
        for i in 0..8 {
            // White pawns
            set_at!(pawns, pos_to_index(i, 1));
            set_at!(whites, pos_to_index(i, 1));
            // Black pawns
            set_at!(pawns, pos_to_index(i, 6));
        }

        // Kings
        set_at!(kings, pos_to_index(4, 0));
        set_at!(whites, pos_to_index(4, 0));
        set_at!(kings, pos_to_index(4, 7));

        // Queens
        set_at!(queens, pos_to_index(3, 0));
        set_at!(whites, pos_to_index(3, 0));
        set_at!(queens, pos_to_index(3, 7));

        // Rooks
        set_at!(rooks, pos_to_index(0, 0));
        set_at!(rooks, pos_to_index(7, 0));
        set_at!(whites, pos_to_index(0, 0));
        set_at!(whites, pos_to_index(7, 0));
        set_at!(rooks, pos_to_index(0, 7));
        set_at!(rooks, pos_to_index(7, 7));

        // bishops
        set_at!(bishops, pos_to_index(2, 0));
        set_at!(bishops, pos_to_index(5, 0));
        set_at!(whites, pos_to_index(2, 0));
        set_at!(whites, pos_to_index(5, 0));
        set_at!(bishops, pos_to_index(2, 7));
        set_at!(bishops, pos_to_index(5, 7));

        // knights
        set_at!(knights, pos_to_index(1, 0));
        set_at!(knights, pos_to_index(6, 0));
        set_at!(whites, pos_to_index(1, 0));
        set_at!(whites, pos_to_index(6, 0));
        set_at!(knights, pos_to_index(1, 7));
        set_at!(knights, pos_to_index(6, 7));

        ChessGame {
            whites,
            pawns,
            bishops,
            knights,
            rooks,
            queens,
            kings,
            flags: 0,
        }
    }

    /// Parse a game from a FEN description
    ///
    /// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn from_fen(fen: &str) -> ChessGame {
        let mut whites = 0u64;
        let mut pawns = 0u64;
        let mut bishops = 0u64;
        let mut knights = 0u64;
        let mut rooks = 0u64;
        let mut queens = 0u64;
        let mut kings = 0u64;

        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            panic!("Invalid FEN: Not enough parts");
        }

        let board_part = parts[0];

        for (rank_idx, rank) in board_part.split('/').enumerate() {
            let row = 7 - rank_idx; // FEN starts with rank 8 (topmost) and stores ranks top to bottom

            let row = row as ChessPosition;
            let mut col = 0 as ChessPosition;

            for c in rank.chars() {
                match c {
                    '1'..='8' => {
                        col += c.to_digit(10).unwrap() as ChessPosition; // Skip empty squares
                    }
                    'p' => {
                        set_at!(pawns, pos_to_index(col, row));
                        col += 1;
                    }
                    'P' => {
                        set_at!(pawns, pos_to_index(col, row));
                        set_at!(whites, pos_to_index(col, row));
                        col += 1;
                    }
                    'r' => {
                        set_at!(rooks, pos_to_index(col, row));
                        col += 1;
                    }
                    'R' => {
                        set_at!(rooks, pos_to_index(col, row));
                        set_at!(whites, pos_to_index(col, row));
                        col += 1;
                    }
                    'n' => {
                        set_at!(knights, pos_to_index(col, row));
                        col += 1;
                    }
                    'N' => {
                        set_at!(knights, pos_to_index(col, row));
                        set_at!(whites, pos_to_index(col, row));
                        col += 1;
                    }
                    'b' => {
                        set_at!(bishops, pos_to_index(col, row));
                        col += 1;
                    }
                    'B' => {
                        set_at!(bishops, pos_to_index(col, row));
                        set_at!(whites, pos_to_index(col, row));
                        col += 1;
                    }
                    'q' => {
                        set_at!(queens, pos_to_index(col, row));
                        col += 1;
                    }
                    'Q' => {
                        set_at!(queens, pos_to_index(col, row));
                        set_at!(whites, pos_to_index(col, row));
                        col += 1;
                    }
                    'k' => {
                        set_at!(kings, pos_to_index(col, row));
                        col += 1;
                    }
                    'K' => {
                        set_at!(kings, pos_to_index(col, row));
                        set_at!(whites, pos_to_index(col, row));
                        col += 1;
                    }
                    _ => panic!("Invalid FEN: Invalid character '{}'", c),
                }
            }
        }

        ChessGame {
            whites,
            pawns,
            bishops,
            knights,
            rooks,
            queens,
            kings,
            flags: 0, // Flags for castling, active color, etc., can be computed if necessary
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fen_standard_game() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen_game = GameConstructor::from_fen(fen);
        let standard_game = GameConstructor::standard_game();

        assert_eq!(fen_game.whites, standard_game.whites, "Mismatch in whites bitboard");
        assert_eq!(fen_game.pawns, standard_game.pawns, "Mismatch in pawns bitboard");
        assert_eq!(fen_game.bishops, standard_game.bishops, "Mismatch in bishops bitboard");
        assert_eq!(fen_game.knights, standard_game.knights, "Mismatch in knights bitboard");
        assert_eq!(fen_game.rooks, standard_game.rooks, "Mismatch in rooks bitboard");
        assert_eq!(fen_game.queens, standard_game.queens, "Mismatch in queens bitboard");
        assert_eq!(fen_game.kings, standard_game.kings, "Mismatch in kings bitboard");

        fen_game.display()
    }
}

