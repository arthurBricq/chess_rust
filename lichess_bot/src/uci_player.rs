use crate::uci_answers::UciAnswer;
use engine::engine::{Engine, SearchResult};
use engine::iterative_deepening::IterativeDeepeningEngine;
use model::game::ChessGame;
use model::game_constructor::GameConstructor;
use model::moves::Move;
use model::utils::ChessPosition;
use vampirc_uci::{UciMessage, UciMove, UciSquare};

pub(crate) struct UciPlayer {
    game: ChessGame,
    solver: IterativeDeepeningEngine,
    white_to_move: bool,
}

impl UciPlayer {
    pub fn new() -> Self {
        Self {
            game: Default::default(),
            solver: IterativeDeepeningEngine::new(4, 4),
            white_to_move: true,
        }
    }

    pub(crate) fn handle_message(&mut self, m: UciMessage) -> UciAnswer {
        match m {
            UciMessage::Uci => UciAnswer::None,
            UciMessage::IsReady => UciAnswer::EngineReady,
            UciMessage::UciNewGame => {
                self.set_game_to_default();
                UciAnswer::None
            }
            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                // TODO handle different arguments
                if !moves.is_empty() {
                    let best_move = self.play_moves(moves);
                    return UciAnswer::BestMove(best_move);
                }
                UciAnswer::None
            }
            UciMessage::Go { time_control, .. } => {
                // TODO handle settings ?
                UciAnswer::None
            }
            _ => UciAnswer::Debug(format!("Unknown message: {:?}", m)),
        }
    }

    fn set_game_to_default(&mut self) {
        self.game = GameConstructor::standard_game()
    }

    fn play_moves(&mut self, moves: Vec<UciMove>) -> Move {
        for mv in moves {
            let mv = uci_move_to_move(mv, self.white_to_move);
            self.white_to_move = !self.white_to_move;
        }
        // Once all the moves are applied, response with the best move
        let SearchResult { score, best_move } =
            self.solver.find_best_move(self.game, self.white_to_move);
        // TODO error handling should be better than this
        best_move.unwrap()
    }
}

/// Converts `UciSquare` to `ChessPosition`
fn uci_square_to_chess_position(square: UciSquare) -> ChessPosition {
    // UCI files are chars: 'a' -> 0, 'b' -> 1, ..., 'h' -> 7
    let file_index = square.file as u8 - b'a';

    // UCI ranks are 1-based: '1' -> 0, '2' -> 1, ..., '8' -> 7
    let rank_index = square.rank - 1;

    // Combine rank and file into a ChessPosition index (using the 0..63 range)
    (rank_index * 8 + file_index) as ChessPosition
}

/// Converts a `UciMove` into a `Move`
fn uci_move_to_move(uci_move: UciMove, is_white: bool) -> Move {
    Move {
        from: uci_square_to_chess_position(uci_move.from),
        to: uci_square_to_chess_position(uci_move.to),
        is_white,
        quality: Default::default(), // Default quality; modify if needed
    }
}
