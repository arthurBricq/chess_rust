use crate::engine::alpha_beta::AlphaBetaEngine;

/// A search engine which uses iterative deepening to sort the best moves at
/// each level.
pub struct IterativeDeepeningEngine {
    depth: usize,
    extra_depth: usize,
    initial_depth: usize,
    engine: AlphaBetaEngine,
}

impl IterativeDeepeningEngine {






}