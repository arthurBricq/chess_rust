use std::cmp::{max, min};
use std::collections::HashMap;
use model::chess_type::ScoreType;
use model::game::ChessGame;
use model::moves::Move;
use model::moves_container::{MovesContainer, SmartMoveContainer};
use crate::engine::{Engine, SearchResult};

pub struct AlphaBetaEngine {
    depth: usize,
    extra_depth: usize,
    transposition_table: HashMap<ChessGame, ScoreType>,
    killer_moves: HashMap<usize, Vec<Move>>,
}

impl Engine for AlphaBetaEngine {
    fn find_best_move(&mut self, game: ChessGame, white_to_play: bool) -> SearchResult {
        self.reset_killer_moves();
        let result = self.alpha_beta_search(
            game,
            white_to_play,
            0,
            i32::MIN as ScoreType,
            i32::MAX as ScoreType,
            false,
            None,
        );
        result
    }
}

impl AlphaBetaEngine {
    pub fn new(depth: usize, extra_depth: usize) -> Self {
        Self {
            depth,
            extra_depth,
            transposition_table: Default::default(),
            killer_moves: Default::default(),
        }
    }

    #[allow(dead_code)]
    pub fn set_engine_depth(&mut self, depth: usize, extra: usize) {
        self.depth = depth;
        self.extra_depth = extra;
        self.reset_killer_moves()
    }

    fn reset_killer_moves(&mut self) {
        self.killer_moves.clear();
        for i in 0..self.depth + self.extra_depth {
            self.killer_moves.insert(i, vec![]);
        }
    }

    /// Returns the best move found using alpha-beta pruning with
    /// * smart move ordering
    /// * extra depth for captures move only
    ///
    /// Alpha-Beta Pruning: engine stops evaluating a move when at least one possibility has been found
    ///                      that proves the move to be worse than a previously examined move.
    /// * alpha = minimum score that white is assured of
    ///         = worth case for white
    /// * beta  = maximum score that black is assured of
    ///         = worth case of black
    ///
    /// Improvements
    /// * Move ordering : we favor moves that captures
    /// * Iterative deepening : provide a "first line" which even improves the move ordering
    /// * Killer-move heuristic : WIP
    ///
    /// Algorithm taken from
    /// https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning#Pseudocode
    /// (fail-soft variation)
    pub fn alpha_beta_search(
        &mut self,
        game: ChessGame,
        white_to_play: bool,
        depth: usize,
        mut alpha: ScoreType,
        mut beta: ScoreType,
        is_last_move_a_capture: bool,
        first_move_to_evaluate: Option<Move>,
    ) -> SearchResult {
        // Terminal node
        if (!is_last_move_a_capture && depth >= self.depth)
            || (is_last_move_a_capture && depth >= self.depth + self.extra_depth)
            || game.is_finished()
        {
            let s = *self
                .transposition_table
                .entry(game)
                .or_insert_with(|| game.score());
            return SearchResult {
                score: s,
                best_move: None,
            };
        }

        // get the list of available moves
        let mut container = SmartMoveContainer::new();
        game.update_move_container(&mut container, white_to_play);

        // Optionally set the first move
        // (used for iterative deepening)
        if let Some(first_move) = first_move_to_evaluate {
            container.set_first_move(first_move);
        }

        // Adds the killer move potentially found by other branches
        if let Some(moves) = self.killer_moves.get(&depth) {
            for i in 0..min(2, moves.len()) {
                container.add_killer_move(moves[i]);
            }
        }

        let mut score = if white_to_play {
            ScoreType::MIN
        } else {
            ScoreType::MAX
        };
        // TODO is there a way to not keep track of the best move at runtime ?
        let mut best_move = None;

        while container.has_next() {
            let mut new_game = game.clone();
            let m = container.get_next();
            new_game.apply_move_unsafe(&m);

            let result = self.alpha_beta_search(
                new_game,
                !white_to_play,
                depth + 1,
                alpha,
                beta,
                m.is_capture(),
                None,
            );

            if white_to_play {
                // value := max(value, alphabeta(child, depth − 1, α, β, FALSE))
                // α := max(α, value)
                // if value ≥ β then break (* β cutoff *)

                // current_score = max(current_score, result.score);
                if result.score > score {
                    best_move = Some(m);
                    score = result.score;
                }
                alpha = max(alpha, score);
                if score >= beta {
                    // cutoff: remember the "killer move" for future branches
                    self.killer_moves
                        .get_mut(&depth)
                        .expect("The datastructure is always initialized to support this usage")
                        .push(m);
                    break;
                }
            } else {
                // value := min(value, alphabeta(child, depth − 1, α, β, TRUE))
                // β := min(β, value)
                // if value ≤ α then break (* α cutoff *)

                // current_score = min(current_score, result.score);
                if result.score < score {
                    best_move = Some(m);
                    score = result.score;
                }
                beta = min(beta, score);
                if score <= alpha {
                    break;
                }
            }
        }

        // Once we reach this point, we have explored all the possible moves of this branch
        // ==> we know which is the best move
        SearchResult { score, best_move }
    }
}

#[cfg(test)]
/// This module tests several starting positions that are easy.
mod tests {
    use model::chess_type::Type::{King, Knight, Pawn, Rook};
    use model::game::ChessGame;
    use model::moves::Move;
    use model::utils::{chesspos_to_index, index_to_chesspos};
    use crate::alpha_beta::AlphaBetaEngine;
    use crate::engine::Engine;

    #[test]
    /// A test in which white or black can take a pawn
    fn test_simple_engine1() {
        let mut game = ChessGame::empty();
        game.set_piece(Pawn, true, "e4");
        game.set_piece(Pawn, true, "e2");

        game.set_piece(Pawn, false, "d5");
        game.set_piece(Pawn, false, "d7");

        // The game needs to have kings alive
        game.set_piece(King, true, "a2");
        game.set_piece(King, false, "a7");

        let mut engine = AlphaBetaEngine::new(6, 0);

        // If it is white to play, white captures the pawn
        let result = engine.find_best_move(game.clone(), true);
        assert_eq!(
            result.best_move.unwrap().from,
            chesspos_to_index("e4").unwrap()
        );
        assert_eq!(
            result.best_move.unwrap().to,
            chesspos_to_index("d5").unwrap()
        );

        // Same if it is for black
        let result = engine.find_best_move(game.clone(), false);
        assert_eq!(
            result.best_move.unwrap().from,
            chesspos_to_index("d5").unwrap()
        );
        assert_eq!(
            result.best_move.unwrap().to,
            chesspos_to_index("e4").unwrap()
        );
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
        let mut game = ChessGame::empty();
        game.set_piece(King, true, "a2");
        game.set_piece(King, false, "a7");

        // White has one pawn
        game.set_piece(Pawn, true, "e4");

        // Black has two pieces
        game.set_piece(Pawn, false, "d5");
        game.set_piece(Knight, false, "f5");

        let mut engine = Box::new(AlphaBetaEngine::new(6, 0));

        // If it is white to play, it should capture the bishop
        let result = engine.find_best_move(game.clone(), true);
        println!(
            "{} {}",
            index_to_chesspos(result.best_move.unwrap().from),
            index_to_chesspos(result.best_move.unwrap().to)
        );
        assert_eq!(
            result.best_move.unwrap().from,
            chesspos_to_index("e4").unwrap()
        );
        assert_eq!(
            result.best_move.unwrap().to,
            chesspos_to_index("f5").unwrap()
        );

        // If black is playing, it should capture the pawn
        let result = engine.find_best_move(game.clone(), false);
        println!(
            "{} {}",
            index_to_chesspos(result.best_move.unwrap().from),
            index_to_chesspos(result.best_move.unwrap().to)
        );
        assert_eq!(
            result.best_move.unwrap().from,
            chesspos_to_index("d5").unwrap()
        );
        assert_eq!(
            result.best_move.unwrap().to,
            chesspos_to_index("e4").unwrap()
        );
    }

    #[test]
    /// A test in which white can take a pawn, but if it does then it reveals a check
    /// So white is supposed to move the pawn up
    fn test_simple_engine3() {
        let mut game = ChessGame::empty();

        // White has one pawn
        game.set_piece(Pawn, true, "e4");
        game.set_piece(Pawn, true, "d4");
        game.set_piece(Pawn, false, "d5");
        game.set_piece(Pawn, false, "f5");
        game.set_piece(Rook, false, "e8");

        // The game needs to have kings alive
        game.set_piece(King, true, "e2");
        game.set_piece(King, false, "a7");

        let mut engine = AlphaBetaEngine::new(6, 0);
        engine.set_engine_depth(4, 0);

        let valid_white_moves = [
            Move::new(
                chesspos_to_index("e2").unwrap(),
                chesspos_to_index("f3").unwrap(),
                true,
            ),
            Move::new(
                chesspos_to_index("e2").unwrap(),
                chesspos_to_index("d3").unwrap(),
                true,
            ),
            Move::new(
                chesspos_to_index("e4").unwrap(),
                chesspos_to_index("e5").unwrap(),
                true,
            ),
        ];

        // If it is white to play, it should move the pawn up and not capture anything
        let result = engine.find_best_move(game.clone(), true);
        println!(
            "{} {}",
            index_to_chesspos(result.best_move.unwrap().from),
            index_to_chesspos(result.best_move.unwrap().to)
        );
        assert!(valid_white_moves.contains(&result.best_move.unwrap()));

        // If black is playing, it should capture the pawn with a piece
        let result = engine.find_best_move(game.clone(), false);
        println!(
            "{} {}",
            index_to_chesspos(result.best_move.unwrap().from),
            index_to_chesspos(result.best_move.unwrap().to)
        );
        assert_eq!(
            result.best_move.unwrap().to,
            chesspos_to_index("e4").unwrap()
        );
    }

    #[test]
    /// A test in which white can deliver check on the king and get a free piece
    fn test_simple_engine4() {
        let mut game = ChessGame::empty();

        // White has one pawn
        game.set_piece(Pawn, true, "e2");
        game.set_piece(Pawn, true, "d2");
        game.set_piece(Pawn, true, "f2");
        game.set_piece(Rook, true, "h4");

        game.set_piece(Pawn, false, "a7");
        game.set_piece(Rook, false, "f5");

        // The game needs to have kings alive
        game.set_piece(King, true, "e1");
        game.set_piece(King, false, "d5");
        game.block_castling();

        let mut engine = AlphaBetaEngine::new(6, 0);

        // If it is white to play, it should move the pawn up and not capture anything
        let result = engine.find_best_move(game.clone(), true);
        println!(
            "{} {}",
            index_to_chesspos(result.best_move.unwrap().from),
            index_to_chesspos(result.best_move.unwrap().to)
        );
        assert_eq!(
            result.best_move.unwrap().from,
            chesspos_to_index("e2").unwrap()
        );
        assert_eq!(
            result.best_move.unwrap().to,
            chesspos_to_index("e4").unwrap()
        );
    }

    #[test]
    /// A test in which if black moves the left most pawn down, it will lose it.
    /// We want to make sure that black sees this treat.
    fn test_simple_engine5() {
        // This position is the one where black is not supposed to play a5->a4
        let pos1 = ChessGame::new(
            402973695,
            71494648782447360,
            2594073385365405732,
            4755801206503243842,
            9295429630892703873,
            576460752303423496,
            1152921504606846992,
            0,
        );

        let mut engine = AlphaBetaEngine::new(6, 0);
        engine.set_engine_depth(4, 4);

        // What is the best move for black ?
        let _ = engine.find_best_move(pos1.clone(), false);

        // Now we wonder, what is the best move for white, given there is one less depth ?
        let mut pos2 = pos1.clone();
        pos2.apply_move_unsafe(&Move::new(
            chesspos_to_index("a5").unwrap(),
            chesspos_to_index("a4").unwrap(),
            false,
        ));
        engine.set_engine_depth(3, 4);
        let _ = engine.find_best_move(pos2.clone(), true);

        // let's understand why is the move that attacks a4 is not seen as strong
        let mut pos3 = pos2.clone();
        pos3.apply_move_unsafe(&Move::new(
            chesspos_to_index("f1").unwrap(),
            chesspos_to_index("b5").unwrap(),
            true,
        ));
        engine.set_engine_depth(2, 4);
        let _ = engine.find_best_move(pos3.clone(), false);
    }

    #[test]
    fn test_score_with_low_depth() {
        let mut game = ChessGame::empty();
        game.set_piece(King, true, "a2");
        game.set_piece(King, false, "a7");

        // White has one pawn
        game.set_piece(Pawn, true, "e4");

        // Black has two pieces
        game.set_piece(Pawn, false, "d5");
        game.set_piece(Knight, false, "f5");

        let mut engine = AlphaBetaEngine::new(6, 0);
        engine.set_engine_depth(1, 0);

        // Asserts that the black captures the knight
        let result = engine.find_best_move(game, true);
        let best_move = result.best_move.unwrap();
        assert_eq!(chesspos_to_index("e4").unwrap(), best_move.from);
        assert_eq!(chesspos_to_index("f5").unwrap(), best_move.to);
    }
}
