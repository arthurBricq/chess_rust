use crate::model::game::ChessGame;
use crate::model::tools::{pos_to_index, set_at};

pub struct GameConstructor;

impl GameConstructor {
    /// An empty chess game
    #[cfg(test)]
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
}
