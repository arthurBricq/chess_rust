use std::collections::HashMap;
use std::time::Instant;
use crate::model::game::{ChessGame, ScoreType};
use crate::model::moves::Move;
use crate::model::moves_container::{MovesContainer, SortedMovesContainer};

pub struct SearchResult {
    pub score: ScoreType,
    pub best_move: Option<Move>,
}

pub struct Engine {
    depth: usize,
    extra_depth: usize,
    iter: u64,
    transposition_table: HashMap<ChessGame, ScoreType>
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
                score: if white_to_play {s} else { -s },
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
