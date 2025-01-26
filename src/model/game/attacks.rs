use crate::model::game::precomputation::{KING_ATTACK_MASKS, KNIGHT_ATTACK_MASKS, PAWN_ATTACK_MASKS};
use crate::model::game::ChessGame;

trait ChessAttacks {
    /// Returns the list of attack squares
    fn get_attacked_squares(&self, white_playing: bool) -> u64;

    /// By the pawn
    fn get_attacked_squares_pawn(&self, white_playing: bool) -> u64;

    /// Get the squared attacked by the knights in this position.
    fn get_attacked_squares_knight(&self, white_playing: bool) -> u64;

    /// Get the squared attacked by the knights in this position.
    fn get_attacked_squares_king(&self, white_playing: bool) -> u64;
}

impl ChessAttacks for ChessGame {
    fn get_attacked_squares(&self, white_playing: bool) -> u64 {
        // TODO
        // There is a nice suggestion done by GPT at the beginning of this thread:
        // https://chatgpt.com/c/67487aed-1ee0-8011-9c8a-8d90c88513e0
        0
    }

    fn get_attacked_squares_pawn(&self, white_playing: bool) -> u64 {
        let (white_pawn_attacks, black_pawn_attacks) = &*PAWN_ATTACK_MASKS;
        let mut attacks = 0;

        let attack_masks = if white_playing {
            white_pawn_attacks
        } else {
            black_pawn_attacks
        };

        // Iterate over all pawns
        let mut pawns_left = self.pawns;
        while pawns_left != 0 {
            // Get the position of the least significant bit
            let sq = pawns_left.trailing_zeros() as usize;
            // Add attacks for the pawn at sq
            attacks |= attack_masks[sq];
            // Remove the least significant bit
            pawns_left &= pawns_left - 1;
        }

        attacks
    }

    /// Returns the attack squares for knights of the specified player.
    fn get_attacked_squares_knight(&self, white_playing: bool) -> u64 {
        let mut attacks = 0;

        let mut knights_left = self.knights
            & (if white_playing {
                self.whites
            } else {
                !self.whites
            });

        // Iterate over all knights depending on the player's color
        while knights_left != 0 {
            // Get the position of the least significant bit (LSB)
            let sq = knights_left.trailing_zeros() as usize;
            // Add the precomputed knight attacks for this position
            attacks |= KNIGHT_ATTACK_MASKS[sq];
            // Remove the LSB
            knights_left &= knights_left - 1;
        }

        attacks
    }

    fn get_attacked_squares_king(&self, white_playing: bool) -> u64 {
        let mut attacks = 0;

        let mut king_left = self.kings
            & (if white_playing {
                self.whites
            } else {
                !self.whites
            });

        // Iterate over all king depending on the player's color
        while king_left != 0 {
            // Get the position of the least significant bit (LSB)
            let sq = king_left.trailing_zeros() as usize;
            // Add the precomputed knight attacks for this position
            attacks |= KING_ATTACK_MASKS[sq];
            // Remove the LSB
            king_left &= king_left - 1;
        }

        attacks
    }
}

#[cfg(test)]
mod tests {
    use crate::model::chess_type::Type::{King, Knight, Pawn};
    use crate::model::game::attacks::ChessAttacks;
    use crate::model::game_constructor::GameConstructor;
    use crate::model::utils::{chesspos_to_index, IntoChessPosition};

    /// Asserts that if a white pawn is in e4, d5 and f5 are attacked
    #[test]
    fn test_pawn_attacks() {
        // Example pre-setup for the test
        let mut chess_game = GameConstructor::empty();

        // Place a pawn at e4 (28th bit in Little-Endian Rank-File mapping)
        chess_game.set_piece(Pawn, true, "e4");

        let attacks = chess_game.get_attacked_squares_pawn(true); // true for white pawns attacking

        // Bitboard representing d5 and f5 squares
        let expected_attacks = (1 << "d5".into_position()) | (1 << "f5".into_position());

        assert_eq!(
            attacks, expected_attacks,
            "Pawn at e4 should attack d5 and f5, but got incorrect attack squares."
        );
    }

    /// Asserts that a white pawn at the edge of the board (e.g., h4) only attacks one square (g5).
    #[test]
    fn test_pawn_attacks_edge_of_board() {
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Pawn, true, "h4");

        let attacks = chess_game.get_attacked_squares_pawn(true);

        let expected_attacks = 1 << "g5".into_position();
        assert_eq!(
            attacks, expected_attacks,
            "Pawn at h4 should only attack g5, but got incorrect attack squares."
        );
    }

    /// Asserts that a black pawn at the edge of the board (e.g., a5) only attacks one square (b4).
    #[test]
    fn test_black_pawn_attacks_edge_of_board() {
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Pawn, false, "a5");

        let attacks = chess_game.get_attacked_squares_pawn(false);

        let expected_attacks = 1 << "b4".into_position();
        assert_eq!(
            attacks, expected_attacks,
            "Pawn at a5 should only attack b4, but got incorrect attack squares."
        );
    }

    /// Asserts that two white pawns attacking the same squares both contribute to the attack.
    #[test]
    fn test_two_pawns_attacking_same_square() {
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Pawn, true, "e4");
        chess_game.set_piece(Pawn, true, "g4");

        let attacks = chess_game.get_attacked_squares_pawn(true);

        let expected_attacks = (1 << "d5".into_position())
            | (1 << "f5".into_position())
            | (1 << "f5".into_position())
            | (1 << "h5".into_position());
        assert_eq!(
            attacks, expected_attacks,
            "Both pawns at e4 and g4 should contribute to attack squares d5, f5, and h5, but got incorrect attack squares."
        );
    }

    /// Asserts that white pawns attacking on their turn behave correctly.
    #[test]
    fn test_white_pawn_attacks() {
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Pawn, true, "d4");

        let attacks = chess_game.get_attacked_squares_pawn(true);

        let expected_attacks = (1 << "c5".into_position()) | (1 << "e5".into_position());
        assert_eq!(
            attacks, expected_attacks,
            "Pawn at d4 should attack c5 and e5, but got incorrect attack squares."
        );
    }

    /// Tests that a knight in e4 attacks the correct 8 squares.
    #[test]
    fn test_knight_attacks_e4() {
        let mut chess_game = GameConstructor::empty();

        // Place a knight at e4
        chess_game.set_piece(Knight, true, "e4");

        let attacks = chess_game.get_attacked_squares_knight(true);

        // Expected attacks for a knight on e4
        let expected_attacks = (1 << "d2".into_position())
            | (1 << "f2".into_position())
            | (1 << "c3".into_position())
            | (1 << "g3".into_position())
            | (1 << "c5".into_position())
            | (1 << "g5".into_position())
            | (1 << "d6".into_position())
            | (1 << "f6".into_position());

        assert_eq!(
            attacks, expected_attacks,
            "Knight at e4 should attack d2, f2, c3, g3, c5, g5, d6, f6, but got incorrect result."
        );
    }

    /// Tests that a knight on the corners of the board (a1, a8, h1, h8) only attacks 2 squares.
    #[test]
    fn test_knight_attacks_edges() {
        // Test a knight at a1
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Knight, true, "a1");
        let attacks_a1 = chess_game.get_attacked_squares_knight(true);
        let expected_a1 = (1 << "b3".into_position()) | (1 << "c2".into_position());
        assert_eq!(
            attacks_a1, expected_a1,
            "Knight at a1 should only attack b3 and c2, but got incorrect result."
        );

        // Test a knight at a8
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Knight, true, "a8");
        let attacks_a8 = chess_game.get_attacked_squares_knight(true);
        let expected_a8 = (1 << "b6".into_position()) | (1 << "c7".into_position());
        assert_eq!(
            attacks_a8, expected_a8,
            "Knight at a8 should only attack b6 and c7, but got incorrect result."
        );

        // Test a knight at h1
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Knight, true, "h1");
        let attacks_h1 = chess_game.get_attacked_squares_knight(true);
        let expected_h1 = (1 << "g3".into_position()) | (1 << "f2".into_position());
        assert_eq!(
            attacks_h1, expected_h1,
            "Knight at h1 should only attack g3 and f2, but got incorrect result."
        );

        // Test a knight at h8
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Knight, true, "h8");
        let attacks_h8 = chess_game.get_attacked_squares_knight(true);
        let expected_h8 = (1 << "g6".into_position()) | (1 << "f7".into_position());
        assert_eq!(
            attacks_h8, expected_h8,
            "Knight at h8 should only attack g6 and f7, but got incorrect result."
        );
    }

    /// Tests that a king in the center of the board (e.g., e4) attacks all 8 adjoining squares.
    #[test]
    fn test_king_attacks_center() {
        let mut chess_game = GameConstructor::empty();

        // Place a king at e4
        chess_game.set_piece(King, true, "e4");

        let attacks = chess_game.get_attacked_squares_king(true);

        // Expected attacks for a king on e4
        let expected_attacks = (1 << "d3".into_position())
            | (1 << "d4".into_position())
            | (1 << "d5".into_position())
            | (1 << "e3".into_position())
            | (1 << "e5".into_position())
            | (1 << "f3".into_position())
            | (1 << "f4".into_position())
            | (1 << "f5".into_position());

        assert_eq!(
            attacks, expected_attacks,
            "King at e4 should attack all 8 surrounding squares, but got incorrect result."
        );
    }

    /// Tests that a king on the edges (e.g., a4, h4, e1, e8) attacks the appropriate limited squares.
    #[test]
    fn test_king_attacks_edges() {
        // Test a king at a4
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "a4");
        let attacks_a4 = chess_game.get_attacked_squares_king(true);
        let expected_a4 = (1 << "a3".into_position())
            | (1 << "a5".into_position())
            | (1 << "b3".into_position())
            | (1 << "b4".into_position())
            | (1 << "b5".into_position());

        assert_eq!(
            attacks_a4, expected_a4,
            "King at a4 should attack 5 surrounding squares, but got incorrect result."
        );

        // Test a king at h4
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "h4");
        let attacks_h4 = chess_game.get_attacked_squares_king(true);
        let expected_h4 = (1 << "g3".into_position())
            | (1 << "g4".into_position())
            | (1 << "g5".into_position())
            | (1 << "h3".into_position())
            | (1 << "h5".into_position());

        assert_eq!(
            attacks_h4, expected_h4,
            "King at h4 should attack 5 surrounding squares, but got incorrect result."
        );

        // Test a king at e1
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "e1");
        let attacks_e1 = chess_game.get_attacked_squares_king(true);
        let expected_e1 = (1 << "d1".into_position())
            | (1 << "d2".into_position())
            | (1 << "e2".into_position())
            | (1 << "f1".into_position())
            | (1 << "f2".into_position());

        assert_eq!(
            attacks_e1, expected_e1,
            "King at e1 should attack 5 surrounding squares, but got incorrect result."
        );

        // Test a king at e8
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "e8");
        let attacks_e8 = chess_game.get_attacked_squares_king(true);
        let expected_e8 = (1 << "d8".into_position())
            | (1 << "d7".into_position())
            | (1 << "e7".into_position())
            | (1 << "f8".into_position())
            | (1 << "f7".into_position());

        assert_eq!(
            attacks_e8, expected_e8,
            "King at e8 should attack 5 surrounding squares, but got incorrect result."
        );
    }

    /// Tests that a king in a corner (e.g., a1, h1, a8, h8) attacks only the 3 nearby squares.
    #[test]
    fn test_king_attacks_corners() {
        // Test a king at a1
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "a1");
        let attacks_a1 = chess_game.get_attacked_squares_king(true);
        let expected_a1 = (1 << "a2".into_position())
            | (1 << "b1".into_position())
            | (1 << "b2".into_position());

        assert_eq!(
            attacks_a1, expected_a1,
            "King at a1 should only attack 3 surrounding squares, but got incorrect result."
        );

        // Test a king at h1
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "h1");
        let attacks_h1 = chess_game.get_attacked_squares_king(true);
        let expected_h1 = (1 << "h2".into_position())
            | (1 << "g1".into_position())
            | (1 << "g2".into_position());

        assert_eq!(
            attacks_h1, expected_h1,
            "King at h1 should only attack 3 surrounding squares, but got incorrect result."
        );

        // Test a king at a8
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "a8");
        let attacks_a8 = chess_game.get_attacked_squares_king(true);
        let expected_a8 = (1 << "a7".into_position())
            | (1 << "b8".into_position())
            | (1 << "b7".into_position());

        assert_eq!(
            attacks_a8, expected_a8,
            "King at a8 should only attack 3 surrounding squares, but got incorrect result."
        );

        // Test a king at h8
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(King, true, "h8");
        let attacks_h8 = chess_game.get_attacked_squares_king(true);
        let expected_h8 = (1 << "h7".into_position())
            | (1 << "g8".into_position())
            | (1 << "g7".into_position());

        assert_eq!(
            attacks_h8, expected_h8,
            "King at h8 should only attack 3 surrounding squares, but got incorrect result."
        );
    }}
