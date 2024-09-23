use std::time::Instant;

use crate::model::moves_container::{MovesContainer, SortedMovesContainer};

use super::super::model::game::*;
use super::super::model::moves::*;

pub struct SearchResult {
    pub score: ScoreType,
    pub best_move: Option<Move>,
}

pub struct Engine {
    depth: usize, 
    extra_depth: usize,
    iter: u64,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            depth: 4,
            extra_depth: 0,
            iter: 0,
        }
    }
    
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
    fn tree_search(&mut self,
                   game: ChessGame,
                   white_to_play: bool,
                   depth: usize,
                   mut alpha: ScoreType,
                   mut beta: ScoreType,
                   last_move_captured: bool,
    ) -> SearchResult {
        // Ending criteria
        if (!last_move_captured && depth >= self.depth) ||
            (last_move_captured && depth >= self.depth + self.extra_depth) ||
            game.is_finished()
        {
            self.iter += 1;

            return SearchResult {
                score: game.score(),
                best_move: None,
            };
        }

        let mut current_score = if white_to_play {
            ScoreType::MIN
        } else {
            ScoreType::MAX
        };

        // get the list of available moves
        let mut container = SortedMovesContainer::new();
        game.update_move_container(&mut container, white_to_play);

        // The best move is initialized with the first one
        let mut best_move = None;
        
        while container.has_next() {
            let mut new_game = game.clone();
            let m = container.get_next();
            new_game.apply_move_unsafe(&m);

            // call the recursion
            let result = self.tree_search(new_game,
                                          !white_to_play,
                                          depth + 1,
                                          alpha,
                                          beta,
                                          m.is_capture());
            
            // Alpha beta pruning

            if white_to_play {
                // Keep the maximum score
                if result.score > current_score {
                    best_move = Some(m);
                    current_score = result.score;
                }
                // beta cutoff
                if current_score > beta {
                    break;
                }
                // Update alpha: eg, the minimum score that white is guaranteed
                // alpha becomes the max of the score
                if result.score > alpha {
                    alpha = result.score;
                }
            } else {
                if result.score < current_score {
                    best_move = Some(m);
                    current_score = result.score;
                }
                // alpha cutoff
                if current_score < alpha {
                    break;
                }
                if result.score < beta {
                    beta = result.score;
                }
            }
        }


        // Once we reach this point, we have explored all the possible moves of this branch
        // ==> we know which is the best move
        return SearchResult {
            score: current_score, 
            best_move,
        };

    }

    fn alpha_beta_search(&mut self,
                         game: ChessGame,
                         white_to_play: bool,
                         depth: usize,
                         mut alpha: ScoreType,
                         beta: ScoreType,
                         last_move_capture: bool
    ) ->  SearchResult {
        // Ending criteria
        if (!last_move_capture && depth >= self.depth) ||
            (last_move_capture && depth >= self.depth + self.extra_depth) ||
            game.is_finished()
        {
            self.iter += 1;

            return SearchResult {
                score: if white_to_play {game.score()} else { -game.score() },
                // score: if white_to_play {-game.score()} else { game.score() },
                // score: -game.score(),
                best_move: None,
            };
        }

        // get the list of available moves
        let mut container = SortedMovesContainer::new();
        game.update_move_container(&mut container, white_to_play);

        // The best move is initialized with the first one
        let mut current_score = i32::MIN as ScoreType;
        let mut best_move = None;
        
        if depth == 0 {
            println!("current = {current_score}")
        }
        
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

            let s = - result.score;

            if s > current_score {
                best_move = Some(m);
                current_score = s;
            }

            if current_score > alpha {
                alpha = current_score;
            }
            
            if depth == 0 {
                println!("move = {m:?}, s = {s}, current = {current_score}")
            }

            if alpha >= beta {
                if depth == 0 {
                    println!("Cutoff")
                }
                break;
            }
            
        }

        // Once we reach this point, we have explored all the possible moves of this branch
        // ==> we know which is the best move
        return SearchResult {
            score: current_score,
            best_move,
        };
    }
}
