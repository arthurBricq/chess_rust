use crate::model::game::ChessGame;
use crate::model::precomputation::PAWN_ATTACK_MASKS;

trait ChessAttacks {
    /// Returns the list of attack squares
    fn get_attacked_squares(&self, white_playing: bool) -> u64;

    /// By the pawn
    fn get_attacked_squares_pawn(&self, white_playing: bool) -> u64;
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
}

#[cfg(test)]
mod tests {
    use crate::model::chess_type::Type::Pawn;
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

        let expected_attacks =
            (1 << "d5".into_position()) | (1 << "f5".into_position()) | (1 << "f5".into_position()) | (1 << "h5".into_position());
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
}
