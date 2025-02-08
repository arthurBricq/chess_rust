use crate::game::precomputation::{
    KING_ATTACK_MASKS, KNIGHT_ATTACK_MASKS, PAWN_ATTACK_MASKS, SLIDING_ATTACK_MASKS,
};
use crate::game::ChessGame;
use crate::utils::{consume_bits, is_set, pieces_for_color, set_at};
use std::ops::Range;

pub(super) trait ChessAttacks {
    /// Returns the list of attack squares
    fn get_attacked_squares(&self, white_playing: bool) -> u64 {
        self.get_attacked_squares_bishop(white_playing)
            | self.get_attacked_squares_knight(white_playing)
            | self.get_attacked_squares_king(white_playing)
            | self.get_attacked_squares_pawn(white_playing)
            | self.get_attacked_squares_queen(white_playing)
            | self.get_attacked_squares_rook(white_playing)
    }
    /// Get the squared attacked by the pawn
    fn get_attacked_squares_pawn(&self, white_playing: bool) -> u64;
    /// Get the squared attacked by the knights in this position.
    fn get_attacked_squares_knight(&self, white_playing: bool) -> u64;
    /// Get the squared attacked by the knights in this position.
    fn get_attacked_squares_king(&self, white_playing: bool) -> u64;
    /// Get the squared attacked by the rooks
    fn get_attacked_squares_rook(&self, white_playing: bool) -> u64;
    /// Get the squared attacked by the bishops
    fn get_attacked_squares_bishop(&self, white_playing: bool) -> u64;
    /// Get the squared attacked by the queen
    fn get_attacked_squares_queen(&self, white_playing: bool) -> u64;
}

impl ChessGame {
    /// Computes the squares attacked by a sliding piece (rook, bishop, or queen)
    /// on the chessboard.
    ///
    /// Sliding pieces attack squares along straight paths until obstructed
    /// by another piece or reaching the edge of the board. This function uses
    /// precomputed sliding attack masks for each square and direction to determine
    /// the attacks.
    ///
    /// # Arguments
    ///
    /// - `pieces`: A bitboard representing the positions of the sliding pieces whose
    ///   attacks are to be computed.
    /// - `direction_indices`: A range specifying the indices of directions to consider
    ///   in the `SLIDING_ATTACK_MASKS`. For example:
    ///   - `0..4`: Horizontal and vertical directions (rook-like movement).
    ///   - `4..8`: Diagonal directions (bishop-like movement).
    ///
    /// # Returns
    ///
    /// A bitboard representing all squares attacked by the sliding pieces.
    ///
    /// # Details
    ///
    /// The calculation proceeds as follows:
    /// - For each piece in the bitboard, its position is determined.
    /// - For each direction in the given range, the attack ray for this direction
    ///   is retrieved from the precomputed `SLIDING_ATTACK_MASKS`.
    /// - The attack is calculated iteratively until an occupied square is encountered
    ///   (blocking the attack in that direction).
    ///
    /// This method makes use of bitwise operations for efficient computation.
    ///
    /// # Notes
    ///
    /// The method assumes a precomputed occupancy bitboard (`self.rooks | self.kings |
    /// self.queens | self.pawns | self.bishops | self.knights`) to identify blocking pieces.
    ///
    /// If a rook is at "c4", its attack squares in horizontal and vertical directions are computed,
    /// with blocking taken into account appropriately.
    fn get_attacked_squares_from_sliding_piece(
        &self,
        pieces: u64,
        direction_indices: Range<usize>,
    ) -> u64 {
        let mut attacks = 0;
        let occupancy =
            self.rooks | self.kings | self.queens | self.pawns | self.bishops | self.knights;

        consume_bits!(pieces, sq, {
            // For each direction
            for dir in direction_indices.clone() {
                // Get the attack ray for this direction from the precomputed sliding masks
                let ray = &SLIDING_ATTACK_MASKS[dir][sq];
                // Go through all the positions
                for position in ray {
                    set_at!(attacks, *position);
                    // If the square is occupied, end
                    if is_set!(occupancy, *position) {
                        break;
                    }
                }
            }
        });

        attacks
    }
}

impl ChessAttacks for ChessGame {
    fn get_attacked_squares_pawn(&self, white_playing: bool) -> u64 {
        let (white_pawn_attacks, black_pawn_attacks) = &*PAWN_ATTACK_MASKS;
        let attack_masks = if white_playing {
            white_pawn_attacks
        } else {
            black_pawn_attacks
        };

        let mut attacks = 0;
        let pawns_left = pieces_for_color!(self.whites, self.pawns, white_playing);
        consume_bits!(pawns_left, sq, {
            // Add attacks for the pawn at sq
            attacks |= attack_masks[sq];
        });

        attacks
    }

    /// Returns the attack squares for knights of the specified player.
    fn get_attacked_squares_knight(&self, white_playing: bool) -> u64 {
        let mut attacks = 0;
        let knights_left = pieces_for_color!(self.whites, self.knights, white_playing);
        consume_bits!(knights_left, sq, {
            attacks |= KNIGHT_ATTACK_MASKS[sq];
        });
        attacks
    }

    fn get_attacked_squares_king(&self, white_playing: bool) -> u64 {
        let mut attacks = 0;
        let king_left = pieces_for_color!(self.whites, self.kings, white_playing);
        consume_bits!(king_left, sq, {
            attacks |= KING_ATTACK_MASKS[sq];
        });

        attacks
    }

    fn get_attacked_squares_rook(&self, white_playing: bool) -> u64 {
        let rook_left = pieces_for_color!(self.whites, self.rooks, white_playing);
        self.get_attacked_squares_from_sliding_piece(rook_left, 0..4)
    }

    fn get_attacked_squares_bishop(&self, white_playing: bool) -> u64 {
        let bishops_left = pieces_for_color!(self.whites, self.bishops, white_playing);
        self.get_attacked_squares_from_sliding_piece(bishops_left, 4..8)
    }

    fn get_attacked_squares_queen(&self, white_playing: bool) -> u64 {
        let queens = pieces_for_color!(self.whites, self.queens, white_playing);
        self.get_attacked_squares_from_sliding_piece(queens, 0..8)
    }
}



#[cfg(test)]
mod tests {
    use crate::chess_type::Type::{Bishop, King, Knight, Pawn, Rook};
    use crate::game::attacks::ChessAttacks;
    use crate::game::ChessGame;
    use crate::utils::{print_bitboard, IntoChessPosition};

    /// Asserts that if a white pawn is in e4, d5 and f5 are attacked
    #[test]
    fn test_pawn_attacks() {
        // Example pre-setup for the test
        let mut chess_game = ChessGame::empty();

        // Place a pawn at e4 (28th bit in Little-Endian Rank-File mapping)
        chess_game.set_piece(Pawn, true, "e4");

        let attacks = chess_game.get_attacked_squares_pawn(true); // true for white pawns attacking

        // Bitboard representing d5 and f5 squares
        let expected_attacks = (1 << "d5".as_chess_position()) | (1 << "f5".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "Pawn at e4 should attack d5 and f5, but got incorrect attack squares."
        );
    }

    /// Asserts that a white pawn at the edge of the board (e.g., h4) only attacks one square (g5).
    #[test]
    fn test_pawn_attacks_edge_of_board() {
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Pawn, true, "h4");

        let attacks = chess_game.get_attacked_squares_pawn(true);

        let expected_attacks = 1 << "g5".as_chess_position();
        assert_eq!(
            attacks, expected_attacks,
            "Pawn at h4 should only attack g5, but got incorrect attack squares."
        );
    }

    /// Asserts that a black pawn at the edge of the board (e.g., a5) only attacks one square (b4).
    #[test]
    fn test_black_pawn_attacks_edge_of_board() {
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Pawn, false, "a5");

        let attacks = chess_game.get_attacked_squares_pawn(false);

        let expected_attacks = 1 << "b4".as_chess_position();
        assert_eq!(
            attacks, expected_attacks,
            "Pawn at a5 should only attack b4, but got incorrect attack squares."
        );
    }

    /// Asserts that two white pawns attacking the same squares both contribute to the attack.
    #[test]
    fn test_two_pawns_attacking_same_square() {
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Pawn, true, "e4");
        chess_game.set_piece(Pawn, true, "g4");

        let attacks = chess_game.get_attacked_squares_pawn(true);

        let expected_attacks = (1 << "d5".as_chess_position())
            | (1 << "f5".as_chess_position())
            | (1 << "f5".as_chess_position())
            | (1 << "h5".as_chess_position());
        assert_eq!(
            attacks, expected_attacks,
            "Both pawns at e4 and g4 should contribute to attack squares d5, f5, and h5, but got incorrect attack squares."
        );
    }

    /// Asserts that white pawns attacking on their turn behave correctly.
    #[test]
    fn test_white_pawn_attacks() {
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Pawn, true, "d4");

        let attacks = chess_game.get_attacked_squares_pawn(true);

        let expected_attacks = (1 << "c5".as_chess_position()) | (1 << "e5".as_chess_position());
        assert_eq!(
            attacks, expected_attacks,
            "Pawn at d4 should attack c5 and e5, but got incorrect attack squares."
        );
    }

    /// Tests that a knight in e4 attacks the correct 8 squares.
    #[test]
    fn test_knight_attacks_e4() {
        let mut chess_game = ChessGame::empty();

        // Place a knight at e4
        chess_game.set_piece(Knight, true, "e4");

        let attacks = chess_game.get_attacked_squares_knight(true);

        // Expected attacks for a knight on e4
        let expected_attacks = (1 << "d2".as_chess_position())
            | (1 << "f2".as_chess_position())
            | (1 << "c3".as_chess_position())
            | (1 << "g3".as_chess_position())
            | (1 << "c5".as_chess_position())
            | (1 << "g5".as_chess_position())
            | (1 << "d6".as_chess_position())
            | (1 << "f6".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "Knight at e4 should attack d2, f2, c3, g3, c5, g5, d6, f6, but got incorrect result."
        );
    }

    /// Tests that a knight on the corners of the board (a1, a8, h1, h8) only attacks 2 squares.
    #[test]
    fn test_knight_attacks_edges() {
        // Test a knight at a1
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Knight, true, "a1");
        let attacks_a1 = chess_game.get_attacked_squares_knight(true);
        let expected_a1 = (1 << "b3".as_chess_position()) | (1 << "c2".as_chess_position());
        assert_eq!(
            attacks_a1, expected_a1,
            "Knight at a1 should only attack b3 and c2, but got incorrect result."
        );

        // Test a knight at a8
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Knight, true, "a8");
        let attacks_a8 = chess_game.get_attacked_squares_knight(true);
        let expected_a8 = (1 << "b6".as_chess_position()) | (1 << "c7".as_chess_position());
        assert_eq!(
            attacks_a8, expected_a8,
            "Knight at a8 should only attack b6 and c7, but got incorrect result."
        );

        // Test a knight at h1
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Knight, true, "h1");
        let attacks_h1 = chess_game.get_attacked_squares_knight(true);
        let expected_h1 = (1 << "g3".as_chess_position()) | (1 << "f2".as_chess_position());
        assert_eq!(
            attacks_h1, expected_h1,
            "Knight at h1 should only attack g3 and f2, but got incorrect result."
        );

        // Test a knight at h8
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Knight, true, "h8");
        let attacks_h8 = chess_game.get_attacked_squares_knight(true);
        let expected_h8 = (1 << "g6".as_chess_position()) | (1 << "f7".as_chess_position());
        assert_eq!(
            attacks_h8, expected_h8,
            "Knight at h8 should only attack g6 and f7, but got incorrect result."
        );
    }

    /// Tests that a king in the center of the board (e.g., e4) attacks all 8 adjoining squares.
    #[test]
    fn test_king_attacks_center() {
        let mut chess_game = ChessGame::empty();

        // Place a king at e4
        chess_game.set_piece(King, true, "e4");

        let attacks = chess_game.get_attacked_squares_king(true);

        // Expected attacks for a king on e4
        let expected_attacks = (1 << "d3".as_chess_position())
            | (1 << "d4".as_chess_position())
            | (1 << "d5".as_chess_position())
            | (1 << "e3".as_chess_position())
            | (1 << "e5".as_chess_position())
            | (1 << "f3".as_chess_position())
            | (1 << "f4".as_chess_position())
            | (1 << "f5".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "King at e4 should attack all 8 surrounding squares, but got incorrect result."
        );
    }

    /// Tests that a king on the edges (e.g., a4, h4, e1, e8) attacks the appropriate limited squares.
    #[test]
    fn test_king_attacks_edges() {
        // Test a king at a4
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "a4");
        let attacks_a4 = chess_game.get_attacked_squares_king(true);
        let expected_a4 = (1 << "a3".as_chess_position())
            | (1 << "a5".as_chess_position())
            | (1 << "b3".as_chess_position())
            | (1 << "b4".as_chess_position())
            | (1 << "b5".as_chess_position());

        assert_eq!(
            attacks_a4, expected_a4,
            "King at a4 should attack 5 surrounding squares, but got incorrect result."
        );

        // Test a king at h4
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "h4");
        let attacks_h4 = chess_game.get_attacked_squares_king(true);
        let expected_h4 = (1 << "g3".as_chess_position())
            | (1 << "g4".as_chess_position())
            | (1 << "g5".as_chess_position())
            | (1 << "h3".as_chess_position())
            | (1 << "h5".as_chess_position());

        assert_eq!(
            attacks_h4, expected_h4,
            "King at h4 should attack 5 surrounding squares, but got incorrect result."
        );

        // Test a king at e1
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "e1");
        let attacks_e1 = chess_game.get_attacked_squares_king(true);
        let expected_e1 = (1 << "d1".as_chess_position())
            | (1 << "d2".as_chess_position())
            | (1 << "e2".as_chess_position())
            | (1 << "f1".as_chess_position())
            | (1 << "f2".as_chess_position());

        assert_eq!(
            attacks_e1, expected_e1,
            "King at e1 should attack 5 surrounding squares, but got incorrect result."
        );

        // Test a king at e8
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "e8");
        let attacks_e8 = chess_game.get_attacked_squares_king(true);
        let expected_e8 = (1 << "d8".as_chess_position())
            | (1 << "d7".as_chess_position())
            | (1 << "e7".as_chess_position())
            | (1 << "f8".as_chess_position())
            | (1 << "f7".as_chess_position());

        assert_eq!(
            attacks_e8, expected_e8,
            "King at e8 should attack 5 surrounding squares, but got incorrect result."
        );
    }

    /// Tests that a king in a corner (e.g., a1, h1, a8, h8) attacks only the 3 nearby squares.
    #[test]
    fn test_king_attacks_corners() {
        // Test a king at a1
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "a1");
        let attacks_a1 = chess_game.get_attacked_squares_king(true);
        let expected_a1 =
            (1 << "a2".as_chess_position()) | (1 << "b1".as_chess_position()) | (1 << "b2".as_chess_position());

        assert_eq!(
            attacks_a1, expected_a1,
            "King at a1 should only attack 3 surrounding squares, but got incorrect result."
        );

        // Test a king at h1
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "h1");
        let attacks_h1 = chess_game.get_attacked_squares_king(true);
        let expected_h1 =
            (1 << "h2".as_chess_position()) | (1 << "g1".as_chess_position()) | (1 << "g2".as_chess_position());

        assert_eq!(
            attacks_h1, expected_h1,
            "King at h1 should only attack 3 surrounding squares, but got incorrect result."
        );

        // Test a king at a8
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "a8");
        let attacks_a8 = chess_game.get_attacked_squares_king(true);
        let expected_a8 =
            (1 << "a7".as_chess_position()) | (1 << "b8".as_chess_position()) | (1 << "b7".as_chess_position());

        assert_eq!(
            attacks_a8, expected_a8,
            "King at a8 should only attack 3 surrounding squares, but got incorrect result."
        );

        // Test a king at h8
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(King, true, "h8");
        let attacks_h8 = chess_game.get_attacked_squares_king(true);
        let expected_h8 =
            (1 << "h7".as_chess_position()) | (1 << "g8".as_chess_position()) | (1 << "g7".as_chess_position());

        assert_eq!(
            attacks_h8, expected_h8,
            "King at h8 should only attack 3 surrounding squares, but got incorrect result."
        );
    }

    /// Tests that a rook in a1 attacks exactly 14 squares.
    #[test]
    fn test_rook_attacks_a1() {
        let mut chess_game = ChessGame::empty();

        // Place a rook at a1
        chess_game.set_piece(Rook, true, "a1");

        let attacks = chess_game.get_attacked_squares_rook(true);

        print_bitboard(attacks);

        // Expected attacks for a rook on a1
        let expected_attacks = (1 << "a2".as_chess_position())
            | (1 << "a3".as_chess_position())
            | (1 << "a4".as_chess_position())
            | (1 << "a5".as_chess_position())
            | (1 << "a6".as_chess_position())
            | (1 << "a7".as_chess_position())
            | (1 << "a8".as_chess_position())
            | (1 << "b1".as_chess_position())
            | (1 << "c1".as_chess_position())
            | (1 << "d1".as_chess_position())
            | (1 << "e1".as_chess_position())
            | (1 << "f1".as_chess_position())
            | (1 << "g1".as_chess_position())
            | (1 << "h1".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "Rook at a1 should attack exactly 14 squares, but got an incorrect result."
        );
    }

    /// Tests that a rook in a1 attacks correctly when a pawn blocks part of its path at a4.
    #[test]
    fn test_rook_attacks_a1_with_blocking_pawn() {
        let mut chess_game = ChessGame::empty();
        chess_game.set_piece(Rook, true, "a1");
        chess_game.set_piece(Pawn, true, "a2");

        let attacks = chess_game.get_attacked_squares_rook(true);

        // Count the number of 1s in attacks (number of attacked squares)
        let count = attacks.count_ones();
        println!("Number of attacked squares: {}", count);

        // Expected attacks for a rook on a1 when a pawn is blocking at a4
        let expected_attacks = (1 << "a2".as_chess_position())
            | (1 << "b1".as_chess_position())
            | (1 << "c1".as_chess_position())
            | (1 << "d1".as_chess_position())
            | (1 << "e1".as_chess_position())
            | (1 << "f1".as_chess_position())
            | (1 << "g1".as_chess_position())
            | (1 << "h1".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "Rook at a1 should correctly attack up to the blocking pawn at a4 and no further."
        );
    }

    /// Tests that with rooks in a1 and c1, and a pawn in c2, the total number of attacked squares by white rooks is 21.
    #[test]
    fn test_double_rooks_with_blocking_pawn() {
        let mut chess_game = ChessGame::empty();

        // Place rooks at a1 and c1
        chess_game.set_piece(Rook, true, "a1");
        chess_game.set_piece(Rook, true, "c1");

        // Place a blocking pawn at c2
        chess_game.set_piece(Pawn, true, "c2");

        let attacks = chess_game.get_attacked_squares_rook(true);

        // Count the number of attacked squares
        let total_attacked_squares = attacks.count_ones();

        assert_eq!(
            total_attacked_squares, 16,
            "Total number of attacked squares by white rooks should be 15, but got {}.",
            total_attacked_squares
        );
    }

    /// Tests that a white rook at e4 with black pieces at e3, e5, d4, and f4 attacks only 4 squares.
    #[test]
    fn test_rook_e4_with_blocking_pieces() {
        let mut chess_game = ChessGame::empty();

        // Place a white rook at e4
        chess_game.set_piece(Rook, true, "e4");

        // Place black pieces at e3, e5, d4, and f4
        chess_game.set_piece(Pawn, false, "e3");
        chess_game.set_piece(Pawn, false, "e5");
        chess_game.set_piece(Pawn, false, "d4");
        chess_game.set_piece(Pawn, false, "f4");

        let attacks = chess_game.get_attacked_squares_rook(true);

        // Expected attacks for a rook on e4 with blocking pieces
        let expected_attacks = (1 << "e3".as_chess_position())
            | (1 << "e5".as_chess_position())
            | (1 << "d4".as_chess_position())
            | (1 << "f4".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "Rook at e4 should attack only 4 squares when blocked by pieces at e3, e5, d4, and f4."
        );
    }

    /// Tests that a bishop in a1 attacks all squares in its diagonal paths without any blockers.
    #[test]
    fn test_bishop_attacks_a1() {
        let mut chess_game = ChessGame::empty();

        // Place a bishop at a1
        chess_game.set_piece(Bishop, true, "a1");

        let attacks = chess_game.get_attacked_squares_bishop(true);

        // Expected attacks for a bishop on a1
        let expected_attacks = (1 << "b2".as_chess_position())
            | (1 << "c3".as_chess_position())
            | (1 << "d4".as_chess_position())
            | (1 << "e5".as_chess_position())
            | (1 << "f6".as_chess_position())
            | (1 << "g7".as_chess_position())
            | (1 << "h8".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "Bishop at a1 should attack all diagonal squares: b2, c3, d4, etc., but got an incorrect result."
        );
    }

    /// Tests that a bishop in e4 attacks all squares in its diagonal paths without any blockers.
    #[test]
    fn test_bishop_attacks_e4() {
        let mut chess_game = ChessGame::empty();

        // Place a bishop at e4
        chess_game.set_piece(Bishop, true, "e4");

        let attacks = chess_game.get_attacked_squares_bishop(true);

        // Expected attacks for a bishop on e4
        let expected_attacks = (1 << "d3".as_chess_position())
            | (1 << "c2".as_chess_position())
            | (1 << "b1".as_chess_position())
            | (1 << "f3".as_chess_position())
            | (1 << "g2".as_chess_position())
            | (1 << "h1".as_chess_position())
            | (1 << "d5".as_chess_position())
            | (1 << "c6".as_chess_position())
            | (1 << "b7".as_chess_position())
            | (1 << "a8".as_chess_position())
            | (1 << "f5".as_chess_position())
            | (1 << "g6".as_chess_position())
            | (1 << "h7".as_chess_position());

        assert_eq!(
            attacks, expected_attacks,
            "Bishop at e4 should attack both diagonals: d3->b1, f3->h1, d5->a8, and f5->h7, but got an incorrect result."
        );
    }
    
    
    #[test]
    fn test_attacks_of_pawns_at_first_rank() {
        let game = ChessGame::standard_game();
        print_bitboard(game.get_attacked_squares_pawn(true));
        // println!("-----");
        // print_bitboard(game.get_attacked_squares(false));
    }
}
