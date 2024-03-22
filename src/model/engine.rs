use std::cmp;
use std::collections::HashMap;
use super::super::model::game::*;
use super::super::model::moves::*;
use std::time::Instant;

const ENGINE_DEPTH: i8 = 4;
const EXTRA_CAPTURE_MOVE: i8 = 4;

type SearchResult = (ScoreType, Option<Move>);

pub struct Engine {
    iter: u64,
    transposition_table: HashMap<ChessGame, ScoreType>,
    use_transposition: bool
}

impl Engine {
    pub fn new() -> Self {
        Self { iter: 0, transposition_table: HashMap::new(), use_transposition: true}
    }

    /// For a given chessgame, finds the solver's best move and returns it as an Option of a move. 
    /// The function also returns the NPS (nodes per second) in the unit k-nps (for benchmarking)
    pub fn find_best_move(&mut self, game: ChessGame, white_to_play: bool) -> (Option<Move>, u128) {
        self.iter = 0;
        self.transposition_table.clear();
        let start = Instant::now();
        let result = self.tree_search(game, white_to_play, 0, ScoreType::MIN, ScoreType::MAX, false);
        let end = start.elapsed().as_millis() as f64 / 1000.;
        let nps = (self.iter as f64) / end;
        println!("\n\nSolver finished after evaluating {} positions", self.iter);
        println!("    nps = {nps} [moves/second]");
        println!("    transposition table contains {} positions", self.transposition_table.keys().len());
        return (result.1, nps as u128);
    }

    fn tree_search(&mut self,
                   game: ChessGame,
                   white_to_play: bool,
                   depth: i8,
                   mut alpha: ScoreType,
                   mut beta: ScoreType,
                   last_move_captured: bool,
    ) -> SearchResult {
        if depth >= ENGINE_DEPTH {
            if (last_move_captured && depth == ENGINE_DEPTH + EXTRA_CAPTURE_MOVE) || (!last_move_captured) {
                self.iter += 1;
                // Compute score only if it was not computed before
                if self.use_transposition && self.transposition_table.contains_key(&game) {
                    return (*self.transposition_table.get(&game).unwrap(), None)
                } else {
                    let s = game.score();
                    if self.use_transposition {
                        self.transposition_table.insert(game, s);
                    }
                    return (s, None);
                }
            }
        }

        // GOAL 
        let mut current_score = if white_to_play {
            ScoreType::MIN
        } else {
            ScoreType::MAX
        };

        // The best move is initialized with the first one
        let mut best_move = 0;

        // get the list of available moves
        // this is the only time that the function is called
        let moves = game.get_available_moves(white_to_play);

        // for each available move that is also valid, apply this move and run the search to the next depth
        for i in 0..moves.len() {
            // make a copy of the game
            let mut new_game = game.clone();

            // apply the move on the copy, without any kind of check 
            // (because we only have valid moves)
            new_game.apply_move_unsafe(&moves[i]);

            // call the recursion
            let result = self.tree_search(new_game, !white_to_play, depth + 1,
                                          alpha,
                                          beta,
                                          moves[i].is_capture());

            // Update the scores
            if white_to_play {
                if result.0 > current_score {
                    best_move = i;
                    current_score = result.0;
                }
                if result.0 > beta {
                    // stop exploring
                    break;
                }
                if result.0 > alpha {
                    alpha = result.0;
                }
            } else {
                if result.0 < current_score {
                    best_move = i;
                    current_score = result.0;
                }
                if result.0 < alpha {
                    // stop exploring
                    break;
                }
                if result.0 < beta {
                    beta = result.0;
                }
            }
        }

        // Once we reach this point, we know which is the best move. We return the score.

        return if depth == 0 {
            (current_score, Some(moves[best_move]))
        } else {
            (current_score, None)
        }
    }
}
