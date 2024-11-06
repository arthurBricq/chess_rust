use crate::model::chess_type::ScoreType;
use crate::model::game::ChessGame;
use crate::model::moves::Move;
use crate::model::moves_container::{MovesContainer, SortedMovesContainer};
use std::collections::HashMap;
use std::time::Instant;

pub struct SearchResult {
    pub score: ScoreType,
    pub best_move: Option<Move>,
}

pub struct Engine {
    depth: usize,
    extra_depth: usize,
    iter: u64,
    transposition_table: HashMap<ChessGame, ScoreType>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            depth: 6,
            extra_depth: 0,
            iter: 0,
            transposition_table: Default::default(),
        }
    }

    #[cfg(test)]
    pub fn set_engine_depth(&mut self, depth: usize, extra: usize) {
        self.depth = depth;
        self.extra_depth = extra;
    }

    /// For a given chess game, finds the solver's best move and returns it as an Option of a move. 
    /// The function also returns the NPS (nodes per second) in the unit k-nps (for benchmarking)
    pub fn find_best_move(&mut self, game: ChessGame, white_to_play: bool) -> (SearchResult, u128) {
        self.iter = 0;

        let start = Instant::now();
        let result = self.alpha_beta_search(game, white_to_play, 0, i32::MIN as ScoreType, i32::MAX as ScoreType, false);
        let end = start.elapsed().as_millis() as f64 / 1000.;

        let nps = (self.iter as f64) / end;
        println!("\n\nSolver finished after evaluating {} positions", self.iter);
        println!("    score = {} [points]", result.score);
        println!("    time = {end} [second]");
        println!("    nps = {nps} [moves/second]");
        (result, nps as u128)
    }

    /// Chess engine tree search
    ///
    /// Alpha-Beta Pruning: engine stops evaluating a move when at least one possibility has been found
    ///                      that proves the move to be worse than a previously examined move.
    /// * alpha = minimum score that white is assured of
    ///         = worth case for white
    /// * beta  = maximum score that black is assured of
    ///         = worth case of black
    ///
    /// Move ordering : we favor moves that captures
    fn alpha_beta_search(&mut self,
                         game: ChessGame,
                         white_to_play: bool,
                         depth: usize,
                         mut alpha: ScoreType,
                         beta: ScoreType,
                         last_move_capture: bool,
    ) -> SearchResult {
        // Ending criteria
        if (!last_move_capture && depth >= self.depth) ||
            (last_move_capture && depth >= self.depth + self.extra_depth) ||
            game.is_finished()
        {
            self.iter += 1;

            let s = *self.transposition_table.entry(game).or_insert_with(|| game.score());
            return SearchResult {
                score: if white_to_play { s } else { -s },
                best_move: None,
            };
        }

        // get the list of available moves
        let mut container = SortedMovesContainer::new();
        game.update_move_container(&mut container, white_to_play);

        // The best move is initialized with the first one
        let mut current_score = i32::MIN as ScoreType;
        let mut best_move = None;

        while container.has_next() {
            let mut new_game = game.clone();
            let m = container.get_next();
            new_game.apply_move_unsafe(&m);

            // call the recursion
            let result = self.alpha_beta_search(new_game,
                                                !white_to_play,
                                                depth + 1,
                                                -beta,
                                                -alpha,
                                                m.is_capture());

            let s = -result.score;

            if s > current_score {
                best_move = Some(m);
                current_score = s;
            }

            if current_score > alpha {
                alpha = current_score;
            }

            if alpha >= beta {
                break;
            }
        }

        // Once we reach this point, we have explored all the possible moves of this branch
        // ==> we know which is the best move
        SearchResult {
            score: current_score,
            best_move,
        }
    }
}

#[cfg(test)]
/// This module tests several starting positions that are easy.
mod tests {
    use crate::model::engine::Engine;
    use crate::model::chess_type::Type::{King, Knight, Pawn, Rook};
    use crate::model::game::ChessGame;
    use crate::model::game_constructor::GameConstructor;
    use crate::model::moves::Move;
    use crate::model::tools::{chesspos_to_index, index_to_chesspos};

    #[test]
    /// A test in which white or black can take a pawn
    fn test_simple_engine1() {
        let mut game = GameConstructor::empty();
        game.set_piece(Pawn, true, chesspos_to_index("e4").unwrap() as u8);
        game.set_piece(Pawn, true, chesspos_to_index("e2").unwrap() as u8);

        game.set_piece(Pawn, false, chesspos_to_index("d5").unwrap() as u8);
        game.set_piece(Pawn, false, chesspos_to_index("d7").unwrap() as u8);

        // The game needs to have kings alive
        game.set_piece(King, true, chesspos_to_index("a2").unwrap() as u8);
        game.set_piece(King, false, chesspos_to_index("a7").unwrap() as u8);

        let mut engine = Engine::new();

        // If it is white to play, white captures the pawn
        let (result, _) = engine.find_best_move(game.clone(), true);
        assert_eq!(result.best_move.unwrap().from, chesspos_to_index("e4").unwrap());
        assert_eq!(result.best_move.unwrap().to, chesspos_to_index("d5").unwrap());

        // Same if it is for black
        let (result, _) = engine.find_best_move(game.clone(), false);
        assert_eq!(result.best_move.unwrap().from, chesspos_to_index("d5").unwrap());
        assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4").unwrap());
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
        game.set_piece(King, true, chesspos_to_index("a2").unwrap() as u8);
        game.set_piece(King, false, chesspos_to_index("a7").unwrap() as u8);

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e4").unwrap() as u8);

        // Black has two pieces
        game.set_piece(Pawn, false, chesspos_to_index("d5").unwrap() as u8);
        game.set_piece(Knight, false, chesspos_to_index("f5").unwrap() as u8);

        let mut engine = Engine::new();

        // If it is white to play, it should capture the bishop
        let (result, score) = engine.find_best_move(game.clone(), true);
        println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
        assert_eq!(result.best_move.unwrap().from, chesspos_to_index("e4").unwrap());
        assert_eq!(result.best_move.unwrap().to, chesspos_to_index("f5").unwrap());

        // If black is playing, it should capture the pawn
        let (result, score) = engine.find_best_move(game.clone(), false);
        println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
        assert_eq!(result.best_move.unwrap().from, chesspos_to_index("d5").unwrap());
        assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4").unwrap());
    }

    #[test]
    /// A test in which white can take a pawn, but if it does then it reveals a check
    /// So white is supposed to move the pawn up
    fn test_simple_engine3() {
        let mut game = GameConstructor::empty();

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e4").unwrap() as u8);
        game.set_piece(Pawn, true, chesspos_to_index("d4").unwrap() as u8);

        game.set_piece(Pawn, false, chesspos_to_index("d5").unwrap() as u8);
        game.set_piece(Pawn, false, chesspos_to_index("f5").unwrap() as u8);
        game.set_piece(Rook, false, chesspos_to_index("e8").unwrap() as u8);

        // The game needs to have kings alive
        game.set_piece(King, true, chesspos_to_index("e2").unwrap() as u8);
        game.set_piece(King, false, chesspos_to_index("a7").unwrap() as u8);

        let mut engine = Engine::new();
        engine.set_engine_depth(4, 0);

        let valid_white_moves = [
            Move::new(chesspos_to_index("e2").unwrap(), chesspos_to_index("f3").unwrap(), true),
            Move::new(chesspos_to_index("e2").unwrap(), chesspos_to_index("d3").unwrap(), true),
            Move::new(chesspos_to_index("e4").unwrap(), chesspos_to_index("e5").unwrap(), true),
        ];

        // If it is white to play, it should move the pawn up and not capture anything
        let (result, score) = engine.find_best_move(game.clone(), true);
        println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
        assert!(valid_white_moves.contains(&result.best_move.unwrap()));

        // If black is playing, it should capture the pawn with a piece
        let (result, score) = engine.find_best_move(game.clone(), false);
        println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
        assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4").unwrap());
    }

    #[test]
    /// A test in which white can deliver check on the king and get a free piece
    fn test_simple_engine4() {
        let mut game = GameConstructor::empty();

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e2").unwrap() as u8);
        game.set_piece(Pawn, true, chesspos_to_index("d2").unwrap() as u8);
        game.set_piece(Pawn, true, chesspos_to_index("f2").unwrap() as u8);
        game.set_piece(Rook, true, chesspos_to_index("h4").unwrap() as u8);

        game.set_piece(Pawn, false, chesspos_to_index("a7").unwrap() as u8);
        game.set_piece(Rook, false, chesspos_to_index("f5").unwrap() as u8);

        // The game needs to have kings alive
        game.set_piece(King, true, chesspos_to_index("e1").unwrap() as u8);
        game.set_piece(King, false, chesspos_to_index("d5").unwrap() as u8);
        game.block_castling();

        let mut engine = Engine::new();

        // If it is white to play, it should move the pawn up and not capture anything
        let (result, score) = engine.find_best_move(game.clone(), true);
        println!("{} {} --> {score}", index_to_chesspos(result.best_move.unwrap().from), index_to_chesspos(result.best_move.unwrap().to));
        assert_eq!(result.best_move.unwrap().from, chesspos_to_index("e2").unwrap());
        assert_eq!(result.best_move.unwrap().to, chesspos_to_index("e4").unwrap());
    }

    #[test]
    /// A test in which if black moves the left most pawn down, it will lose it.
    /// We want to make sure that black sees this treat.
    fn test_simple_engine5() {
        // This position is the one where black is not supposed to play a5->a4
        let pos1 = ChessGame::new(402973695, 71494648782447360, 2594073385365405732, 4755801206503243842, 9295429630892703873, 576460752303423496, 1152921504606846992, 0);

        let mut engine = Engine::new();
        engine.set_engine_depth(4, 4);

        // What is the best move for black ?
        let (_, _) = engine.find_best_move(pos1.clone(), false);

        // Now we wonder, what is the best move for white, given there is one less depth ?
        let mut pos2 = pos1.clone();
        pos2.apply_move_unsafe(&Move::new(chesspos_to_index("a5").unwrap(), chesspos_to_index("a4").unwrap(), false));
        engine.set_engine_depth(3, 4);
        let (_, _) = engine.find_best_move(pos2.clone(), true);

        // let's understand why is the move that attacks a4 is not seen as strong
        let mut pos3 = pos2.clone();
        pos3.apply_move_unsafe(&Move::new(chesspos_to_index("f1").unwrap(), chesspos_to_index("b5").unwrap(), true));
        engine.set_engine_depth(2, 4);
        let (_, _) = engine.find_best_move(pos3.clone(), false);
    }

    #[test]
    fn test_score_with_low_depth() {
        let mut game = GameConstructor::empty();
        game.set_piece(King, true, chesspos_to_index("a2").unwrap() as u8);
        game.set_piece(King, false, chesspos_to_index("a7").unwrap() as u8);

        // White has one pawn
        game.set_piece(Pawn, true, chesspos_to_index("e4").unwrap() as u8);

        // Black has two pieces
        game.set_piece(Pawn, false, chesspos_to_index("d5").unwrap() as u8);
        game.set_piece(Knight, false, chesspos_to_index("f5").unwrap() as u8);

        let mut engine = Engine::new();
        engine.set_engine_depth(1, 0);

        // The result must be zero after white capture
        let pos1 = game.clone();
        let (result, _) = engine.find_best_move(pos1.clone(), true);
        assert_eq!(0, result.score);
    }
}
