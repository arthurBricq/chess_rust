#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::hash::Hash;
    use crate::model::game::{ChessGame, ScoreType};
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

    #[test]
    fn test_simple_hash_map() {
        let mut map: HashMap<i8, i8> = HashMap::new();
        map.insert(1, 12);
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), false);
        println!("{:?}", map);
    }

    #[test]
    fn test_transposition_table() {
        let mut map: HashMap<ChessGame, ScoreType> = HashMap::new();
        // Create some games
        let mut g1 = ChessGame::new();
        let mut g2 = ChessGame::new();
        let mut g3 = ChessGame::new();
        g1.apply_move_safe(Move::new(12, 28));
        g3.apply_move_safe(Move::new(12, 28));
        // Fill the transposition table with two positions
        map.insert(g1, g1.score());
        map.insert(g2, g2.score());
        // Assert that g3 does not need a score computation
        assert!(map.contains_key(&g1));
        assert!(map.contains_key(&g3));
        // Assert that g3 needs a score computation
        g3.apply_move_safe(Move::new(11, 27));
        assert!(!map.contains_key(&g3));
    }
    
    #[test]
    fn test_score_at_beggining() {
        let game = ChessGame::new();
        assert_eq!(0, game.score());
    }

}