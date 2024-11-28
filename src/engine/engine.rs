use crate::model::chess_type::ScoreType;
use crate::model::game::ChessGame;
use crate::model::moves::Move;

pub struct SearchResult {
    pub score: ScoreType,
    pub best_move: Option<Move>,
}

pub trait Engine {
    /// For a given chess game, finds the solver's best move and returns it as an Option of a move.
    /// The function also returns the NPS (nodes per second) in the unit k-nps (for benchmarking)
    fn find_best_move(&mut self, game: ChessGame, white_to_play: bool) -> SearchResult;
}
