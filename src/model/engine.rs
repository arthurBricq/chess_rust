use std::collections::{HashMap, HashSet};
use super::super::model::game::*;
use super::super::model::moves::*;
use std::time::Instant;
use crate::model::engine::MoveOrderState::{AcceptsOnlyCapture, Finished, AcceptsAllMove};

enum MoveOrderState {
    AcceptsOnlyCapture,
    AcceptsAllMove,
    Finished,
}

impl MoveOrderState {
    fn next(&mut self) {
        match self {
            AcceptsOnlyCapture => *self = AcceptsAllMove,
            AcceptsAllMove => *self = Finished,
            _ => panic!("We must not arrive here")
        }
    }

    /// Returns true if the move is accepted at this stage of the looping through moves
    fn accepts(&self, m: &Move) -> bool {
        match self {
            AcceptsOnlyCapture => m.is_capture(),
            AcceptsAllMove => true,
            _ => panic!("We must not arrive here")
        }
    }
}

pub struct SearchResult {
    score: ScoreType,
    best_move: Option<Move>,
}

pub struct Engine {
    depth: usize, 
    extra_depth: usize,
    iter: u64,
    transposition_table: HashMap<ChessGame, ScoreType>,
    use_transposition: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self { depth: 6, extra_depth: 2, iter: 0, transposition_table: HashMap::new(), use_transposition: true }
    }
    
    pub fn set_engine_depth(&mut self, depth: usize, extra: usize) {
        self.depth = depth;
        self.extra_depth = extra;
    }

    /// For a given chess game, finds the solver's best move and returns it as an Option of a move. 
    /// The function also returns the NPS (nodes per second) in the unit k-nps (for benchmarking)
    pub fn find_best_move(&mut self, game: ChessGame, white_to_play: bool) -> (Option<Move>, u128) {
        self.iter = 0;
        self.transposition_table.clear();
        let start = Instant::now();
        let result = self.tree_search(game, white_to_play, 0, ScoreType::MIN, ScoreType::MAX, false);
        let end = start.elapsed().as_millis() as f64 / 1000.;
        let nps = (self.iter as f64) / end;
        println!("\n\nSolver finished after evaluating {} positions", self.iter);
        println!("    score = {} [points]", result.score);
        println!("    time = {end} [second]");
        println!("    nps = {nps} [moves/second]");
        println!("    transposition table contains {} positions", self.transposition_table.keys().len());
        return (result.best_move, nps as u128);
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
            (game.is_finished())
        {
            self.iter += 1;
            
            // Compute score only if it was not computed before
            let score = if self.use_transposition {
                if self.transposition_table.contains_key(&game) {
                    *self.transposition_table.get(&game).unwrap()
                } else {
                    let s = game.score();
                    self.transposition_table.insert(game, s);
                    s
                }
            } else {
               game.score() 
            };
            
            return SearchResult {
                score,
                best_move: None,
            };
        }

        let mut current_score = if white_to_play {
            ScoreType::MIN
        } else {
            ScoreType::MAX
        };

        // get the list of available moves
        // this is the only time that the function is called
        let moves = game.get_available_moves(white_to_play);
        
        // The best move is initialized with the first one
        let mut best_move = 0;

        // A small state machine is used to pass through moves in a specific order
        // This is why the code is quite hard to read. 
        // This state machine allows to avoid sorting moves, but I am not sure if this was a good decision.
        // Instead of sorting moves, the `move_index` passes through the all list of moves several times and each time only 
        // one type of move is accepted.
        let mut move_index = 0;
        let mut move_state = AcceptsOnlyCapture;
        let mut visited_moves: HashSet<usize> = HashSet::with_capacity(moves.len());

        loop {
            if !visited_moves.contains(&move_index) && move_state.accepts(&moves[move_index]) {
                visited_moves.insert(move_index);

                if depth == 0 {
                    println!("move = {:?}", moves[move_index]);
                }

                // TODO maybe these two functions can be squashed into a single one
                let mut new_game = game.clone();
                new_game.apply_move_unsafe(&moves[move_index]);

                // call the recursion
                let result = self.tree_search(new_game,
                                              !white_to_play,
                                              depth + 1,
                                              alpha,
                                              beta,
                                              moves[move_index].is_capture());

                if depth == 0 {
                    println!("   score = {:?}, move = {:?}", result.score, result.best_move);
                }

                // Alpha beta pruning
                if white_to_play {
                    // Keep the maximum score
                    if result.score > current_score {
                        best_move = move_index;
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
                        best_move = move_index;
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

            // Break when we have gone through all moves
            if visited_moves.len() == moves.len() {
                break;
            }

            // When the move index reaches the end of the list of moves, apply the following
            // state of the engine state machine
            move_index += 1;
            if move_index == moves.len() {
                move_state.next();
                match move_state {
                    AcceptsOnlyCapture | AcceptsAllMove => move_index = 0,
                    Finished => break
                }
            }
        }

        // Once we reach this point, we have explored all the possible moves of this branch
        // ==> we know which is the best move
        return SearchResult {
            score: current_score, 
            best_move: Some(moves[best_move]),
        };

        // return if depth == 0 {
        //     (current_score, Some(moves[best_move]))
        // } else {
        //     (current_score, None)
        // };
    }
}
