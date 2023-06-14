use super::super::model::engine::Engine;
use super::super::model::moves::Move;
use crate::model::game::{ChessGame, Type, pos_to_index};

#[derive(Copy, Clone)]
pub enum Msg {
    RestartGame,
    SquareTapped(i8),
}

pub enum SquareType {
    Attacked,
    Idle,
    LastEngineMove,
}

pub struct ChessViewModel {
    game: ChessGame,
    selected_pos: Option<i8>,
    engine_move: Option<(i8, i8)>,
}

/// A chessview is a class responsible for drawing a chess game.
pub trait ChessView {
    fn refresh(&mut self);
    fn play(&mut self);
}


impl ChessViewModel
{
    pub fn new() -> Self {
        Self {
            game: ChessGame::new(),
            selected_pos: None,
            engine_move: None,
        }
    }

    pub fn get_image_name_at(&self, i: i8, j: i8) -> Option<String> {
        if let Some(t) = self.game.type_at(i, j) {
            if self.game.is_white_at(i, j) {
                match t {
                    Type::Pawn => { Some("pawn_white.svg".to_string()) }
                    Type::Bishop => { Some("bishop_white.svg".to_string()) }
                    Type::Knight => { Some("knight_white.svg".to_string()) }
                    Type::Rook => { Some("rook_white.svg".to_string()) }
                    Type::Queen => { Some("queen_white.svg".to_string()) }
                    Type::King => { Some("king_white.svg".to_string()) }
                }
            } else {
                match t {
                    Type::Pawn => { Some("pawn_dark.svg".to_string()) }
                    Type::Bishop => { Some("bishop_dark.svg".to_string()) }
                    Type::Knight => { Some("knight_dark.svg".to_string()) }
                    Type::Rook => { Some("rook_dark.svg".to_string()) }
                    Type::Queen => { Some("queen_dark.svg".to_string()) }
                    Type::King => { Some("king_dark.svg".to_string()) }
                }
            }
        } else {
            None
        }
    }


    pub fn get_char_at(&self, i: i8, j: i8) -> String {
        if let Some(t) = self.game.type_at(i, j) {
            if self.game.is_white_at(i, j) {
                match t {
                    Type::Pawn => { "♙".to_string() }
                    Type::Bishop => { "♗".to_string() }
                    Type::Knight => { "♘".to_string() }
                    Type::Rook => { "♖".to_string() }
                    Type::Queen => { "♕".to_string() }
                    Type::King => { "♔".to_string() }
                }
            } else {
                match t {
                    Type::Pawn => { "♟︎".to_string() }
                    Type::Bishop => { "♝".to_string() }
                    Type::Knight => { "♞".to_string() }
                    Type::Rook => { "♜".to_string() }
                    Type::Queen => { "♛".to_string() }
                    Type::King => { "♚".to_string() }
                }
            }
        } else {
            " ".to_string()
        }
    }

    pub fn is_attacked_at(&self, i: i8, j: i8) -> bool {
        if let Some(pos) = self.selected_pos {
            let mut moves: Vec<Move> = Vec::new();
            let attacked = self.game.fill_possible_moves_from(&mut moves, pos);

            let to_check = pos_to_index(i, j);
            if attacked.contains(&to_check) {
                return true;
            }
        }
        return false;
    }

    pub fn get_class_name(&self, i: i8, j: i8) -> String {
        if self.is_attacked_at(i, j) {
            "attacked".to_string()
        } else {
            "idle".to_string()
        }
    }

    pub fn get_square_type(&self, i: i8, j: i8) -> SquareType {
        if self.is_attacked_at(i, j) {
            return SquareType::Attacked;
        } else {
            if let Some((from, to)) = self.engine_move {
                let pos = pos_to_index(i, j);
                if pos == from || pos == to {
                    return SquareType::LastEngineMove;
                }
            }
        }
        return SquareType::Idle;
    }

    pub fn play_with_engine(&mut self) -> bool {
        // Make the engine play
        let mut solver = Engine::new();
        if let (Some(best_move), _nps) = solver.find_best_move(self.game, false) {
            // Save the move
            self.engine_move = Some((best_move.from, best_move.to));
            // Apply the move
            let success = self.game.apply_move_safe(
                Move::new(best_move.from, best_move.to)
            );
            return success;
        } else {
            return false;
        }
    }

    pub fn message_received(&mut self, msg: &Msg) -> bool {
        match msg {
            Msg::RestartGame => {
                self.game = ChessGame::new();
                return true;
            }

            Msg::SquareTapped(pos) => {
                if self.selected_pos == None {
                    self.selected_pos = Some(*pos);
                } else {
                    self.engine_move = None;

                    let success = self.game.apply_move_safe(
                        Move::new(self.selected_pos.unwrap(), *pos)
                    );

                    if success {
                        // reset the selected position
                        self.selected_pos = None;

                        self.play_with_engine();
                    } else {
                        self.selected_pos = Some(*pos);
                    }
                }

                return true;
            }
        }
    }
}

