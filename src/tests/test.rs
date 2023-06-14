#[cfg(test)]
mod tests {
    use crate::model::game::ChessGame;
    use crate::model::moves::Move;

    #[test]
    fn test_simple_motions() {
        // Create a new chess game
        let mut game = ChessGame::new();

        // A stupid move should not pass
        let m = Move::new(10, 11);
        let mut result = game.apply_move_safe(m);
        assert_eq!(result, false);

        // e4 is a valid move
        result = game.apply_move_safe(Move::new(12, 28));
        assert_eq!(result, true);

        // my chess representation does not keep track of who is the current player
        // e5 for white is a valid move
        result = game.apply_move_safe(Move::new(28, 36));
        assert_eq!(result, true);

        // e5 for black is not valid
        result = game.apply_move_safe(Move::new(52, 36));
        assert_eq!(result, false);

        // e6 for black is valid
        result = game.apply_move_safe(Move::new(52, 44));
        assert_eq!(result, true);
    }

}