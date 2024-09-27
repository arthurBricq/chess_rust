use crate::model::game::{ChessGame, Type};
use crate::model::moves::Move;
use crate::model::moves_container::MovesContainer;

macro_rules! is_set {
    ($a: expr, $at: expr) => {
        (($a >> $at) & 1u64) == 1
    };
}



pub trait MotionIterator {
    fn next(&mut self, game: &ChessGame) -> Option<Move>;
}

pub struct StepMotionIterator {
    /// Origin of the iterator
    from: i8,
    /// Direction of increase
    inc: i8,
    /// Is the white player playing
    is_white: bool,
    /// Type of the piece being moved
    t: Type,
    /// Keeps track if the last move was a capture
    found_capture: bool,
    /// Current position
    pos: i8,
}

impl StepMotionIterator {
    pub fn new(from: i8, inc: i8, is_white: bool, t: Type) -> Self {
        Self {
            from,
            pos: from,
            inc,
            is_white,
            found_capture: false,
            t
        }
    }
}

impl MotionIterator for StepMotionIterator {
    fn next(&mut self, game: &ChessGame) -> Option<Move> {
        if self.found_capture {
            return None;
        }

        // Increase the position
        self.pos += self.inc;

        // Check that are still in the game
        if self.pos < 0 || self.pos >= 64 {
            return None;
        }

        let mut m = Move::new(self.from, self.pos, self.is_white);

        // Check direction
        if game.is_destination_of_incorrect_color(&m) {
            return None;
        }

        // Check boundary overflow
        if !game.is_in_bound(&m, &self.t) {
            return None;
        }

        // Check if it is a capture
        if let Some(captured) = game.type_at_index(m.to) {
            // Keep track that this direction is finished
            self.found_capture = true;
            let piece = game.type_at_index(m.from).unwrap();
            m.set_quality_from_scores(piece.score(), captured.score());
        }

        Some(m)
    }
}





