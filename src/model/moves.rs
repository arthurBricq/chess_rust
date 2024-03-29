use super::game::{Type, }; 
use std::fmt;

pub const PAWN_MOVES: [i8;8] = [
    8, 9, 7,
    -8, -9, -7,
    16, -16,
    ]; 
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

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MoveQuality {
    Capture, 
    Motion,
}

#[derive(Copy, Clone)]
pub struct Move {
    pub from: i8,  // start position
    pub to: i8,    // end position
    pub quality: MoveQuality,
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
    pub fn new(from: i8, to: i8) -> Self {
        Self {from, to, quality: MoveQuality::Motion}
    }

    pub fn set_as_capture(&mut self) {
        self.quality = MoveQuality::Capture;
    }

    pub fn is_capture(&self) -> bool {
        self.quality == MoveQuality::Capture
    }

    pub fn get_vector(&self) -> [i8; 2] {
        let remain = self.from % 8 ; 
        let min = self.from - remain ; 
        let max = self.from + (7-remain) ; 
        if self.to >= min && self.to <= max {
            return [self.to-self.from,0]; 
        } else {
            // Then look up and down
            return [0,0]; 
        }
    }
    
    /// Returns true if the move respect the most basic rules of chess: is it a valid move for the given type.
    /// 
    /// TODO: evaluate the impact of which method is used. Indeed, two methods are possible during runtime
    /// - Using the contains method to check if the diff vector is in the list of allowed mode
    /// - Using arithmetic operations to do the same but with modulo operation. I think this could be better actually.
    pub fn is_legal(&self, t: &Type) -> bool {
        // Boundary conditions
        if self.to >= 64 {
            return false; 
        }

        // The piece has to move
        // NOTE: maybe this may be stupid 
        if self.to == self.from {
            return false; 
        }

        // Compute the vector between the two positions.
        // It has to be of type i16 to handle the case where the piece goes down.
        let diff = self.to - self.from ; 

        // Check if this vector is part of the valid moves for this piece
        match t {
            Type::Pawn => PAWN_MOVES.contains(&diff),
            Type::Rook => ROOK_MOVES.contains(&diff),
            Type::Bishop => BISHOP_MOVES.contains(&diff),
            Type::Knight => KNIGHT_MOVES.contains(&diff),
            Type::Queen => {
                BISHOP_MOVES.contains(&diff) || ROOK_MOVES.contains(&diff)
            },
            Type::King => KING_MOVES.contains(&diff),
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
                return 1; 
            } else {
                return -1; 
            }
        } else {
            // Non horizontal moves are divided in two cases
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

