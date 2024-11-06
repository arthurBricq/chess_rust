#[cfg(test)]
/// This module tests several starting positions that are easy.
mod tests {
    use crate::model::engine::Engine;
    use crate::model::chess_type::Type::{King, Knight, Pawn, Rook};
    use crate::model::game::ChessGame;
    use crate::model::game_constructor::GameConstructor;
    use crate::model::moves::Move;
    use crate::model::tools::{chesspos_to_index, index_to_chesspos};
    use crate::view::terminal_display::TerminalChessView;

    #[test]
    /// A test in which white or black can take a pawn
    fn test_simple_engine1() {
        let mut game = GameConstructor::empty();
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
        if let (result, score) = engine.find_best_move(game.clone(), true) {
            assert_eq!(result.best_move.unwrap().from, chesspos_to_index("e4"));
            assert_eq!(result.best_move.unwrap().to, chesspos_to_index("d5"));
        } else {
            panic!("Error")
        }

        // Same if it is for black
        if let (result, score) = engine.find_best_move(game.clone(), false) {
            assert_eq!(result.best_move.unwrap().from, chesspos_to_index("d5"));
            assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4"));
        } else {
            panic!("Error")
        }
    }

    #[test]
    fn test_integers_operation() {
        let a = i32::MIN as i64;
        let b = i32::MAX as i64;
        let c = -a;
        let d = -b;

        println!("{a}");
        println!("{d}");
        println!("{b}");
        println!("{c}");
    }


    #[test]
    /// A test in which white can take a pawn or a bishop
    fn test_simple_engine2() {
        let mut game = GameConstructor::empty();
        game.set_piece(King, true, chesspos_to_index("a2") as u8);
        game.set_piece(King, false, chesspos_to_index("a7") as u8);

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e4") as u8);

        // Black has two pieces
        game.set_piece(Pawn, false, chesspos_to_index("d5") as u8);
        game.set_piece(Knight, false, chesspos_to_index("f5") as u8);

        let display = TerminalChessView::new(&mut game);
        display.display();

        let mut engine = Engine::new();

        // If it is white to play, it should capture the bishop
        if let (result, score) = engine.find_best_move(game.clone(), true) {
            println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            assert_eq!(result.best_move.unwrap().from, chesspos_to_index("e4"));
            assert_eq!(result.best_move.unwrap().to, chesspos_to_index("f5"));
        } else {
            panic!("Error")
        }

        // If black is playing, it should capture the pawn
        if let (result, score) = engine.find_best_move(game.clone(), false) {
            println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            assert_eq!(result.best_move.unwrap().from, chesspos_to_index("d5"));
            assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4"));
        } else {
            panic!("Error")
        }
    }

    #[test]
    /// A test in which white can take a pawn, but if it does then it reveals a check
    /// So white is supposed to move the pawn up
    fn test_simple_engine3() {
        let mut game = GameConstructor::empty();

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
        engine.set_engine_depth(4, 0);

        let valid_white_moves = [
            Move::new(chesspos_to_index("e2"), chesspos_to_index("f3"), true),
            Move::new(chesspos_to_index("e2"), chesspos_to_index("d3"), true),
            Move::new(chesspos_to_index("e4"), chesspos_to_index("e5"), true),
        ];

        // If it is white to play, it should move the pawn up and not capture anything
        if let (result, score) = engine.find_best_move(game.clone(), true) {
            println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            assert!(valid_white_moves.contains(&result.best_move.unwrap()));
        } else {
            panic!("Error")
        }

        // If black is playing, it should capture the pawn with a piece
        if let (result, score) = engine.find_best_move(game.clone(), false) {
            println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4"));
        } else {
            panic!("Error")
        }
    }

    #[test]
    /// A test in which white can deliver check on the king and get a free piece
    fn test_simple_engine4() {
        let mut game = GameConstructor::empty();

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
        if let (result, score) = engine.find_best_move(game.clone(), true) {
            println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            assert_eq!(result.best_move.unwrap().from, chesspos_to_index("e2"));
            assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4"));
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

        let mut engine = Engine::new();
        engine.set_engine_depth(4, 4);

        // What is the best move for black ?
        if let (result, _) = engine.find_best_move(pos1.clone(), false) {
            println!("{} {}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            println!("-----------\n")
        }


        // Now we wonder, what is the best move for white, given there is one less depth ?
        let mut pos2 = pos1.clone();
        pos2.apply_move_unsafe(&Move::new(chesspos_to_index("a5"), chesspos_to_index("a4"), false));
        engine.set_engine_depth(3, 4);
        if let (result, _) = engine.find_best_move(pos2.clone(), true) {
            println!("{} {}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            println!("-----------\n")
        }

        // let's understand why is the move that attacks a4 is not seen as strong
        let mut pos3 = pos2.clone();
        pos3.apply_move_unsafe(&Move::new(chesspos_to_index("f1"), chesspos_to_index("b5"), true));
        engine.set_engine_depth(2, 4);
        if let (result, _) = engine.find_best_move(pos3.clone(), false) {
            println!("{} {}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            println!("-----------\n")
        }


        // If it is white to play, it should move the pawn up and not capture anything
        // println!("Evaluating pos3");
        // if let (Some(result.best_move.unwrap()), _) = engine.find_best_move(pos3.clone(), false) {
        //     println!("{} {}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
        // } 
    }

    #[test]
    fn test_score_with_low_depth() {
        let mut game = GameConstructor::empty();
        game.set_piece(King, true, chesspos_to_index("a2") as u8);
        game.set_piece(King, false, chesspos_to_index("a7") as u8);

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e4") as u8);

        // Black has two pieces
        game.set_piece(Pawn, false, chesspos_to_index("d5") as u8);
        game.set_piece(Knight, false, chesspos_to_index("f5") as u8);

        let mut engine = Engine::new();
        engine.set_engine_depth(1, 0);

        let pos1 = game.clone();
        if let (result, _) = engine.find_best_move(pos1.clone(), true) {
            println!("{} {}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
            // The result must be zero after white capture
            assert_eq!(0, result.score);
        }
    }
}
