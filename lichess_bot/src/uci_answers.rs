use vampirc_uci::{UciMove, UciSquare};
use model::moves::Move;
use model::utils::{index_to_chesspos, ChessPosition};

pub(crate) enum UciAnswer {
    None,
    Debug(String),
    EngineReady,
    BestMove(Move)
}


impl UciAnswer {
    /// Consumes self and returns the formatted in two parts: 
    /// (1) the part to be answers as per the UCI protocol
    /// (2) a debug line 
    pub(crate) fn into_formatted(self) -> (Option<String>, Option<String>) {
        match self {
            UciAnswer::None => (None, None),
            UciAnswer::Debug(message) => (None, Some(message)),
            UciAnswer::EngineReady => (Some("readyok".to_string()), None),
            UciAnswer::BestMove(mv) => {
                let from = index_to_chesspos(mv.from);
                let to = index_to_chesspos(mv.to);
                (Some(format!("bestmove {from}{to}")), None)
            }
        }
    }
}
