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
    pub(crate) fn into_formatted(self) -> Option<String> {
        match self {
            UciAnswer::None => None,
            UciAnswer::Debug(message) => Some(message),
            UciAnswer::EngineReady => Some("readyok".to_string()),
            UciAnswer::BestMove(mv) => {
                let from = index_to_chesspos(mv.from);
                let to = index_to_chesspos(mv.to);
                Some(format!("bestmove {from}{to}"))
            }
        }
    }
}
