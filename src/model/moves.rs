use super::game::index_to_chesspos;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub const WHITE_PAWN_MOVES: [i8;4] = [8, 9, 7, 16];

pub const BLACK_PAWN_MOVES: [i8;4] = [-8, -9, -7, -16];

pub const ROOK_MOVES: [i8;28] = [
    1,2,3,4,5,6,7,
    -1,-2,-3,-4,-5,-6,-7,
    8,8*2,8*3,8*4,8*5,8*6,8*7,
    -8,-8*2,-8*3,-8*4,-8*5,-8*6,-8*7,
    ];

pub const BISHOP_MOVES: [i8; 28] = [
    7,7*2,7*3,7*4,7*5,7*6,7*7,
    -7,-7*2,-7*3,-7*4,-7*5,-7*6,-7*7,
    9,9*2,9*3,9*4,9*5,9*6,9*7,
    -9,-9*2,-9*3,-9*4,-9*5,-9*6,-9*7,
];

pub const KNIGHT_MOVES: [i8; 8] = [
    17, 15, -15, -17,
    10, -6, 6, -10
];

pub const KING_MOVES: [i8; 8] = [
    1, -1, 8, -8,
    9, 7, -9, -7
];

pub const QUEEN_MOVES: [i8;56] = [
    1,2,3,4,5,6,7,
    -1,-2,-3,-4,-5,-6,-7,
    8,8*2,8*3,8*4,8*5,8*6,8*7,
    -8,-8*2,-8*3,-8*4,-8*5,-8*6,-8*7,
    7,7*2,7*3,7*4,7*5,7*6,7*7,
    -7,-7*2,-7*3,-7*4,-7*5,-7*6,-7*7,
    9,9*2,9*3,9*4,9*5,9*6,9*7,
    -9,-9*2,-9*3,-9*4,-9*5,-9*6,-9*7,
];

pub const KING_SPECIAL_MOVES: [i8; 2] = [
    2, -2 
];

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MoveQuality {
    GoodCapture,
    EqualCapture, 
    LowCapture,
    Motion,
}

#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub from: i8,
    pub to: i8,
    pub is_white: bool,
    pub quality: MoveQuality,
}

impl Debug for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{}", index_to_chesspos(self.from), index_to_chesspos(self.to))
    }
}

impl fmt::Display for Move {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} to {}", self.from, self.to)
    }
}

impl Move {
    pub fn new(from: i8, to: i8, is_white: bool) -> Self {
        Self {from, to, is_white, quality: MoveQuality::Motion}
    }
    
    pub fn set_quality(&mut self, q: MoveQuality) {
        self.quality = q;
    }

    pub fn is_capture(&self) -> bool {
        self.quality == MoveQuality::GoodCapture || 
            self.quality == MoveQuality::EqualCapture || 
            self.quality == MoveQuality::LowCapture
    }

    pub fn get_vector(&self) -> [i8; 2] {
        let remain = self.from % 8 ; 
        let min = self.from - remain ; 
        let max = self.from + (7-remain) ; 
        if self.to >= min && self.to <= max {
            [self.to-self.from,0]
        } else {
            // Then look up and down
            [0,0]
        }
    }

    /// Returns the increment that represents the direction of the given move
    /// This increment is used to traverse all squares between the from and to pos
    /// in order to check if there is another piece in between
    pub fn get_direction_increment(&self) -> i16 {
        let diff = self.to - self.from; 

        // Look on the same line
        let remain = self.from % 8 ; 
        let min = self.from - remain ; 
        let max = self.from + (7-remain) ; 
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
            } 
            else {
                // println!("WARNING: asked for the direction of a move for which is not well defined"); 
                // println!("Move was: {} to {}, diff = {}", self.from, self.to, diff); 
                0
            }
        }
        
    }



}

