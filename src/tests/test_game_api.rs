#[cfg(test)]
mod tests {
    use crate::model::game::{ChessGame, chesspos_to_index};
    use crate::model::game::Type::Pawn;
    use crate::model::moves_container::{MovesContainer, SortedMovesContainer};

    #[test]
    fn test_moves_container_with_basic_position() {
        let mut game = ChessGame::empty();
        game.set_piece(Pawn, true, chesspos_to_index("e2") as u8);
        game.set_piece(Pawn, false, chesspos_to_index("e7") as u8);
        
        let mut container = SortedMovesContainer::new();
        
        game.update_move_container(&mut container, true);
        assert_eq!(2, container.count());
        
        game.update_move_container(&mut container, false);
        assert_eq!(2, container.count());
    }

    #[test]
    fn test_moves_container_with_standard_position() {
        let mut game = ChessGame::standard_game();

        let mut container = SortedMovesContainer::new();

        // there are twenty possible positions
        
        game.update_move_container(&mut container, true);
        assert_eq!(20, container.count());

        game.update_move_container(&mut container, false);
        assert_eq!(20, container.count());
    }
}
