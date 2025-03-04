use crate::uci_answers::UciAnswer;
use engine::engine::{Engine, SearchResult};
use engine::iterative_deepening::IterativeDeepeningEngine;
use model::game::ChessGame;
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
            solver: IterativeDeepeningEngine::new(7, 0),
            white_to_move: true,
        }
    }

    pub(crate) fn handle_message(&mut self, m: UciMessage) -> UciAnswer {
        match m {
            UciMessage::Uci => UciAnswer::Initialize,
            UciMessage::IsReady => UciAnswer::EngineReady,
            UciMessage::Quit => std::process::exit(0),
            UciMessage::UciNewGame => {
                self.set_game_to_default();
                UciAnswer::None
            }
            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                if startpos {
                    self.set_game_to_default();
                }

                if let Some(fen) = fen {
                    self.game = ChessGame::from_fen(fen.as_str());
                }

                self.play_moves(moves);
                UciAnswer::BestMove(self.find_best_move())
            }
            UciMessage::Go { .. } => {
                // TODO handle settings ?
                UciAnswer::None
            }
            _ => UciAnswer::Debug(format!("Unknown message: {:?}", m)),
        }
    }

    fn set_game_to_default(&mut self) {
        self.game = ChessGame::standard_game();
        self.white_to_move = true;
    }

    fn play_moves(&mut self, moves: Vec<UciMove>) {
        for mv in moves {
            let mv = uci_move_to_move(mv, self.white_to_move);
            self.game.apply_move_unsafe(&mv);
            self.white_to_move = !self.white_to_move;
        }
    }

    fn find_best_move(&mut self) -> Move {
        // Once all the moves are applied, response with the best move
        let SearchResult { score: _, best_move } =
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

#[cfg(test)]
mod tests {
    use crate::uci_player::UciPlayer;
    use vampirc_uci::parse;
    use model::utils::index_to_chesspos;
    use crate::uci_answers::UciAnswer;

    #[test]
    fn test_simple_position() {
        let command = "position startpos moves e2e4 e7e6 d2d4";
        let commands = parse(command);
        let mut uci_player = UciPlayer::new();
        let last_answer = commands
            .into_iter()
            .map(|m| uci_player.handle_message(m))
            .last()
            .expect("No answer");

        match last_answer {
            UciAnswer::BestMove(m) => {
                println!("Best move: {}{}", index_to_chesspos(m.from), index_to_chesspos(m.to));
                assert!(uci_player.game.is_black_at(m.from));
            }
            _ => panic!("Expecting a best move, got: {:?}", last_answer),
        }
        
        uci_player.game.display();


    }

}
