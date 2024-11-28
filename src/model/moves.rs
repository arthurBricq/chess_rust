use std::cmp::Ordering;
use crate::model::chess_type::ScoreType;
use crate::model::moves::MoveQuality::{EqualCapture, GoodCapture, LowCapture, Principal, KillerMove, Motion};
use crate::model::utils::{index_to_chesspos, ChessPosition};
use std::fmt;
use std::fmt::{Debug, Formatter};

pub const WHITE_PAWN_MOVES: [i8; 4] = [8, 9, 7, 16];

pub const BLACK_PAWN_MOVES: [i8; 4] = [-8, -9, -7, -16];

pub const KNIGHT_MOVES: [i8; 8] = [
    17, 15, -15, -17,
    10, -6, 6, -10
];

pub const KING_MOVES: [i8; 8] = [
    1, -1, 8, -8,
    9, 7, -9, -7
];

pub const KING_SPECIAL_MOVES: [i8; 2] = [
    2, -2
];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveQuality {
    Principal,
    KillerMove,
    GoodCapture,
    EqualCapture,
    LowCapture,
    Motion,
}

#[derive(Copy, Clone, Eq)]
pub struct Move {
    pub from: ChessPosition,
    pub to: ChessPosition,
    pub is_white: bool,
    pub quality: MoveQuality,
}

impl PartialEq<Self> for Move {
    fn eq(&self, other: &Self) -> bool {
        // The implementation of `PartialEq` is a bit more minimalist than the default
        self.from == other.from && self.to == other.to
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.from, self.to)
    }
}

impl fmt::Display for Move {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}->{}", index_to_chesspos(self.from), index_to_chesspos(self.to))
    }
}

impl Move {
    pub fn new(from: ChessPosition, to: ChessPosition, is_white: bool) -> Self {
        Self { from, to, is_white, quality: MoveQuality::Motion }
    }

    pub fn set_quality(&mut self, q: MoveQuality) {
        self.quality = q;
    }

    pub fn set_quality_from_scores(&mut self, piece: ScoreType, captured: ScoreType) {
        if piece < captured {
            self.set_quality(GoodCapture);
        } else if piece == captured {
            self.set_quality(EqualCapture)
        } else {
            self.set_quality(LowCapture)
        }
    }

    pub fn is_capture(&self) -> bool {
        self.quality == GoodCapture ||
            self.quality == EqualCapture ||
            self.quality == LowCapture
    }

    /// Returns the increment that represents the direction of the given move
    /// This increment is used to traverse all squares between the from and to pos
    /// in order to check if there is another piece in between
    pub fn get_direction_increment(&self) -> i8 {
        let diff = self.to - self.from;

        // Look on the same line
        let remain = self.from % 8;
        let min = self.from - remain;
        let max = self.from + (7 - remain);
        if self.to >= min && self.to <= max {
            // Horizontal moves
            if diff > 0 {
                1
            } else {
                -1
            }
        } else {
            // Non-horizontal moves are divided in two cases
            if diff % 8 == 0 {
                // Vertical moves
                if diff > 0 {
                    8
                } else {
                    -8
                }
            } else if diff % 9 == 0 {
                if diff > 0 {
                    9
                } else {
                    -9
                }
            } else if diff % 7 == 0 {
                if diff > 0 {
                    7
                } else {
                    -7
                }
            } else {
                // println!("WARNING: asked for the direction of a move for which is not well defined"); 
                // println!("Move was: {} to {}, diff = {}", self.from, self.to, diff); 
                0
            }
        }
    }
}

impl From<&MoveQuality> for u8 {
    fn from(value: &MoveQuality) -> Self {
        match value {
            Principal => 5,
            KillerMove => 4,
            GoodCapture => 3,
            EqualCapture => 2,
            LowCapture => 1,
            Motion => 0
        }
    }
}

impl PartialOrd for MoveQuality {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MoveQuality {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::from(self).cmp(&u8::from(other))
    }
}


impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.quality.cmp(&other.quality)
    }
}
