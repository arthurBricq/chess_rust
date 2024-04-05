#[cfg(test)]
/// This module tests several starting positions that are easy.
mod tests {
    use crate::model::engine::Engine;
    use crate::model::game::{ChessGame, chesspos_to_index, index_to_chesspos, pos_to_index};
    use crate::model::game::Type::{Bishop, King, Knight, Pawn, Rook};
    use crate::model::moves::Move;
    use crate::view::terminal_display::TerminalChessView;


    #[test]
    /// A test in which white or black can take a pawn
    fn test_simple_engine1() {
        let mut game = ChessGame::empty();
        game.set_piece(Pawn, true, chesspos_to_index("e4") as u8);
        game.set_piece(Pawn, true, chesspos_to_index("e2") as u8);

        game.set_piece(Pawn, false, chesspos_to_index("d5") as u8);
        game.set_piece(Pawn, false, chesspos_to_index("d7") as u8);

        // The game needs to have kings alive
        game.set_piece(King, true, chesspos_to_index("a2") as u8);
        game.set_piece(King, false, chesspos_to_index("a7") as u8);

        let display = TerminalChessView::new(&mut game);
        display.display();

        let mut engine = Engine::new();

        // If it is white to play, white captures the pawn
        if let (Some(m), score) = engine.find_best_move(game.clone(), true) {
            assert_eq!(m.from, chesspos_to_index("e4"));
            assert_eq!(m.to, chesspos_to_index("d5"));
        } else {
            panic!("Error")
        }

        // Same if it is for black
        if let (Some(m), score) = engine.find_best_move(game.clone(), false) {
            assert_eq!(m.from, chesspos_to_index("d5"));
            assert_eq!(m.to, chesspos_to_index("e4"));
        } else {
            panic!("Error")
        }
    }


    #[test]
    /// A test in which white can take a pawn or a bishop
    fn test_simple_engine2() {
        let mut game = ChessGame::empty();

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e4") as u8);

        game.set_piece(Pawn, false, chesspos_to_index("d5") as u8);
        game.set_piece(Knight, false, chesspos_to_index("f5") as u8);

        // The game needs to have kings alive
        game.set_piece(King, true, chesspos_to_index("a2") as u8);
        game.set_piece(King, false, chesspos_to_index("a7") as u8);

        let display = TerminalChessView::new(&mut game);
        display.display();

        let mut engine = Engine::new();

        // If it is white to play, it should capture the bishop
        if let (Some(m), score) = engine.find_best_move(game.clone(), true) {
            println!("{} {} --> {score}", index_to_chesspos(m.from), index_to_chesspos(m.to));
            assert_eq!(m.from, chesspos_to_index("e4"));
            assert_eq!(m.to, chesspos_to_index("f5"));
        } else {
            panic!("Error")
        }

        // If black is playing, it should capture the pawn
        if let (Some(m), score) = engine.find_best_move(game.clone(), false) {
            println!("{} {} --> {score}", index_to_chesspos(m.from), index_to_chesspos(m.to));
            assert_eq!(m.from, chesspos_to_index("d5"));
            assert_eq!(m.to, chesspos_to_index("e4"));
        } else {
            panic!("Error")
        }
    }

    #[test]
    /// A test in which white can take a pawn, but if it does then it reveals a check
    /// So white is supposed to move the pawn up
    fn test_simple_engine3() {
        let mut game = ChessGame::empty();

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e4") as u8);
        game.set_piece(Pawn, true, chesspos_to_index("d4") as u8);

        game.set_piece(Pawn, false, chesspos_to_index("d5") as u8);
        game.set_piece(Pawn, false, chesspos_to_index("f5") as u8);
        game.set_piece(Rook, false, chesspos_to_index("e8") as u8);

        // The game needs to have kings alive
        game.set_piece(King, true, chesspos_to_index("e2") as u8);
        game.set_piece(King, false, chesspos_to_index("a7") as u8);

        let display = TerminalChessView::new(&mut game);
        display.display();

        let mut engine = Engine::new();

        // If it is white to play, it should move the pawn up and not capture anything
        if let (Some(m), score) = engine.find_best_move(game.clone(), true) {
            println!("{} {} --> {score}", index_to_chesspos(m.from), index_to_chesspos(m.to));
            assert_eq!(m.from, chesspos_to_index("e4"));
            assert_eq!(m.to, chesspos_to_index("e5"));
        } else {
            panic!("Error")
        }

        // If black is playing, it should capture the pawn with a piece
        if let (Some(m), score) = engine.find_best_move(game.clone(), false) {
            println!("{} {} --> {score}", index_to_chesspos(m.from), index_to_chesspos(m.to));
            assert_eq!(m.to, chesspos_to_index("e4"));
        } else {
            panic!("Error")
        }
    }

    #[test]
    /// A test in which white can deliver check on the king and get a free piece
    fn test_simple_engine4() {
        let mut game = ChessGame::empty();

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e2") as u8);
        game.set_piece(Pawn, true, chesspos_to_index("d2") as u8);
        game.set_piece(Pawn, true, chesspos_to_index("f2") as u8);
        game.set_piece(Rook, true, chesspos_to_index("h4") as u8);

        game.set_piece(Pawn, false, chesspos_to_index("a7") as u8);
        game.set_piece(Rook, false, chesspos_to_index("f5") as u8);

        // The game needs to have kings alive
        game.set_piece(King, true, chesspos_to_index("e1") as u8);
        game.set_piece(King, false, chesspos_to_index("d5") as u8);
        game.block_castling();

        let display = TerminalChessView::new(&mut game);
        display.display();

        let mut engine = Engine::new();

        // If it is white to play, it should move the pawn up and not capture anything
        if let (Some(m), score) = engine.find_best_move(game.clone(), true) {
            println!("{} {} --> {score}", index_to_chesspos(m.from), index_to_chesspos(m.to));
            assert_eq!(m.from, chesspos_to_index("e2"));
            assert_eq!(m.to, chesspos_to_index("e4"));
        } else {
            panic!("Error")
        }
    }

    #[test]
    /// A test in which if black moves the left most pawn down, it will lose it.
    /// We want to make sure that black sees this treat.
    fn test_simple_engine5() {
        // This position is the one where black is not supposed to play a5->a4
        let mut pos1 = ChessGame::new(402973695, 71494648782447360, 2594073385365405732, 4755801206503243842, 9295429630892703873, 576460752303423496, 1152921504606846992, 0);
        
        let mut pos2 = pos1.clone();
        pos2.apply_move_unsafe(&Move::new(chesspos_to_index("a5"), chesspos_to_index("a4")));
        
        let mut pos3 = pos2.clone();
        pos3.apply_move_unsafe(&Move::new(chesspos_to_index("f1"), chesspos_to_index("b5")));
        
        // This position is achieved after black plays a5->a4
        // let mut pos2 = ChessGame::new(402973695, 71494644504257280, 2594073385365405732, 4755801206503243842, 9295429630892703873, 576460752303423496, 1152921504606846992, 0);

        // let display = TerminalChessView::new(&mut pos1);
        // display.display();

        let mut engine = Engine::new();
        
        println!("Evaluating pos2");
        if let (Some(m), _) = engine.find_best_move(pos2.clone(), true) {
            println!("{} {}", index_to_chesspos(m.from), index_to_chesspos(m.to));
        }

        // If it is white to play, it should move the pawn up and not capture anything
        println!("Evaluating pos3");
        if let (Some(m), _) = engine.find_best_move(pos3.clone(), false) {
            println!("{} {}", index_to_chesspos(m.from), index_to_chesspos(m.to));
        } 
    }
}
