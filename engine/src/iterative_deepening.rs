use model::chess_type::ScoreType;
use model::game::ChessGame;
use crate::alpha_beta::AlphaBetaEngine;
use crate::engine::{Engine, SearchResult};

/// A search engine which uses iterative deepening to sort the best moves at
/// each level.
pub struct IterativeDeepeningEngine {
    depth: usize,
    extra_depth: usize,
    initial_depth: usize,
}

impl Engine for IterativeDeepeningEngine {
    fn find_best_move(&mut self, game: ChessGame, white_to_play: bool) -> SearchResult {
        let mut search_engine = AlphaBetaEngine::new(6, 0);
        let mut first_move = None;

        let mut depth = self.initial_depth;
        loop {
            search_engine.set_engine_depth(depth, self.extra_depth);
            let result = search_engine.alpha_beta_search(
                game,
                white_to_play,
                0,
                i32::MIN as ScoreType,
                i32::MAX as ScoreType,
                false,
                first_move,
            );

            if depth == self.depth {
                return result;
            }

            first_move = result.best_move;
            depth += 1;
        }
    }
}

impl IterativeDeepeningEngine {
    pub fn new(depth: usize, extra_depth: usize) -> Self {
        Self {
            depth,
            extra_depth,
            initial_depth: 1,
        }
    }
}
