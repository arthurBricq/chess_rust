use super::moves::*;

// transforms a position (x,y) into a bit index
pub fn pos_to_index(x: i8, y: i8) -> i8 {
    x + 8 * y
}

pub fn is_set(a: u64, at: i8) -> bool {
    ((a >> at) & 1u64) == 1
}

/// Returns the position of ones in the provided int
pub fn find_ones(num: u64) -> Vec<i8> {
    let mut indices = Vec::new();

    for i in 0..64 {
        if num & (1 << i) != 0 {
            indices.push(i);
        }
    }

    indices
}

#[derive(PartialEq, Eq)]
pub enum Type {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub type ScoreType = i32;

impl Type {
    pub fn score(&self) -> ScoreType {
        match self {
            Type::Pawn => 1,
            Type::Bishop | Type::Knight => 3,
            Type::Rook => 5,
            Type::Queen => 10,
            Type::King => 10000
        }
    }
}

/// Struct to represent a chess game. 
///
/// Each int represent a type. A '1' value set in each bit means that there is a piece of this type at the position (i % 8, i // 8).
/// Colors of pieces are encoded within the int 'whites'. 
/// An additional int is provided, 'flags', and contained the following information
///     0: has white king moved
///     1: has black king moved
///     2: has white king castled
///     3: has black king castled
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ChessGame {
    whites: u64,
    pawns: u64,
    bishops: u64,
    knights: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
    flags: u64,
}

const FLAG_WK_MOVED: i8 = 0;
const FLAG_BK_MOVED: i8 = 1;
const FLAG_WK_CASTLED: i8 = 2;
const FLAG_BK_CASTLED: i8 = 3;

// Macros are used to work with integers
macro_rules! set_at {
    ($a:expr , $b:expr) => (
        $a |= 1u64 << $b; 
    )
}

macro_rules! clear_at {
    ($a:expr , $b:expr) => (
        $a &= !(1u64 << $b); 
    )
}


impl ChessGame {
    /// Constructor for a normal chess game.
    /// The pieces are set like on a normal chess set.
    pub fn new() -> Self {
        let mut whites = 0;
        let mut pawns = 0;
        let mut bishops = 0;
        let mut knights = 0;
        let mut rooks = 0;
        let mut queens = 0;
        let mut kings = 0;

        // Add the pawns
        for i in 0..8 {
            // White pawns
            set_at!(pawns, pos_to_index(i, 1));
            set_at!(whites, pos_to_index(i, 1));
            // Black pawns
            set_at!(pawns, pos_to_index(i, 6));
        }

        // Kings
        set_at!(kings, pos_to_index(4, 0));
        set_at!(whites, pos_to_index(4, 0));
        set_at!(kings, pos_to_index(4, 7));

        // Queens
        set_at!(queens, pos_to_index(3, 0));
        set_at!(whites, pos_to_index(3, 0));
        set_at!(queens, pos_to_index(3, 7));

        // Rooks 
        set_at!(rooks, pos_to_index(0, 0));
        set_at!(rooks, pos_to_index(7, 0));
        set_at!(whites, pos_to_index(0, 0));
        set_at!(whites, pos_to_index(7, 0));
        set_at!(rooks, pos_to_index(0, 7));
        set_at!(rooks, pos_to_index(7, 7));

        // bishops
        set_at!(bishops, pos_to_index(2, 0));
        set_at!(bishops, pos_to_index(5, 0));
        set_at!(whites, pos_to_index(2, 0));
        set_at!(whites, pos_to_index(5, 0));
        set_at!(bishops, pos_to_index(2, 7));
        set_at!(bishops, pos_to_index(5, 7));

        // knights 
        set_at!(knights, pos_to_index(1,0));
        set_at!(knights, pos_to_index(6,0));
        set_at!(whites, pos_to_index(1,0));
        set_at!(whites, pos_to_index(6,0));
        set_at!(knights, pos_to_index(1,7));
        set_at!(knights, pos_to_index(6,7));

        return Self {
            whites,
            pawns,
            bishops,
            knights,
            rooks,
            queens,
            kings,
            flags: 0,
        };
    }

    /// Returns the type of the provided position.
    /// If no type is present, returns None.
    pub fn type_at(&self, x: i8, y: i8) -> Option<Type> {
        self.type_at_index(pos_to_index(x, y))
    }

    pub fn is_white_at(&self, x: i8, y: i8) -> bool {
        is_set(self.whites, pos_to_index(x, y))
    }

    /// Returns the type of the provided index. 
    /// If no type is present, returns None.
    fn type_at_index(&self, at: i8) -> Option<Type> {
        if is_set(self.pawns, at) {
            Some(Type::Pawn)
        } else if is_set(self.bishops, at) {
            Some(Type::Bishop)
        } else if is_set(self.knights, at) {
            Some(Type::Knight)
        } else if is_set(self.rooks, at) {
            Some(Type::Rook)
        } else if is_set(self.kings, at) {
            Some(Type::King)
        } else if is_set(self.queens, at) {
            Some(Type::Queen)
        } else {
            None
        }
    }

    /// Returns true if there is a piece at this position
    fn has_piece_at(&self, at: i8) -> bool {
        is_set(self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings, at)
    }

    pub fn apply_capture(&mut self, m: &Move) {
        // We can simply clear the position for all integers
        // TODO: evaluate if this approach is not more time consuming than checking all the different integers 
        // and clearing just the correct one
        clear_at!(self.pawns, m.to);
        clear_at!(self.bishops, m.to);
        clear_at!(self.rooks, m.to);
        clear_at!(self.knights, m.to);
        clear_at!(self.queens, m.to);
        clear_at!(self.kings, m.to);
        clear_at!(self.whites, m.to);
    }

    /// Returns true if the provided move does not goes accross another piece.
    fn is_move_over_free_squares(&self, m: &Move) -> bool {
        let direction = m.get_direction_increment();
        if direction == 0 {
            // TODO: ?
            return true;
        }
        let mut current_pos = m.from as i16;
        current_pos += direction;
        while current_pos != m.to as i16 {
            if self.has_piece_at(current_pos as i8) {
                return false;
            }
            current_pos += direction;
        }
        return true;
    }

    /// Returns true if the final square does not have the same color as the origin color
    fn valid_destination(&self, m: &mut Move) -> bool {
        // In the case where there is a piece at the last position, check that it has a different color
        if self.has_piece_at(m.to) {
            let to_is_white = is_set(self.whites, m.to);
            let from_is_white = is_set(self.whites, m.from);
            if to_is_white == from_is_white {
                return false;
            } else {
                m.increase_qualiy();
            }
        }
        return true;
    }

    /// Returns true if the destination is in bound
    fn is_in_bound(&self, m: &Move, t: &Type) -> bool {
        let motion = m.to - m.from;
        let (x1, y1, x2, y2) = (m.from % 8, m.from / 8, m.to % 8, m.to / 8);

        fn is_bishop_valid(motion: &i8, x1: &i8, y1: &i8, x2: &i8, y2: &i8) -> bool {
            if motion % 7 == 0 {
                if *motion > 0 {
                    // upper left diagonal
                    return x1 > x2 && y1 < y2;
                } else {
                    // lower right diagonal
                    return x1 < x2 && y1 > y2;
                }
            } else if motion % 9 == 0 {
                if *motion > 0 {
                    // upper right diagonal
                    return x1 < x2 && y1 < y2;
                } else {
                    // lower left diagonal
                    return x1 > x2 && y1 > y2;
                }
            }
            return false;
        }

        fn is_rook_valid(motion: &i8, m: &Move) -> bool {
            // Look on the same line
            let remain = m.from % 8;
            let min = m.from - remain;
            let max = m.from + (7 - remain);
            if m.to >= min && m.to <= max {
                return true;
            } else {
                // Then look up and down
                return motion % 8 == 0;
            }
        }

        match t {
            Type::Pawn => {
                return (x2 - x1).abs() <= 1;
            }
            Type::Bishop => {
                return is_bishop_valid(&motion, &x1, &y1, &x2, &y2);
            }
            Type::Rook => {
                return is_rook_valid(&motion, &m);
            }
            Type::Queen => {
                return is_bishop_valid(&motion, &x1, &y1, &x2, &y2) || is_rook_valid(&motion, &m);
            }
            Type::Knight => {
                match motion {
                    17 | 10 => return x2 > x1 && y2 > y1,
                    15 | 6 => return x2 < x1 && y2 > y1,
                    -10 | -17 => return x2 < x1 && y2 < y1,
                    -15 | -6 => return x2 > x1 && y2 < y1,
                    _ => return false
                }
            }
            Type::King => {
                match motion {
                    7 | 8 | 9 => if !(y2 > y1) { return false; },
                    -7 | -8 | -9 => if !(y2 < y1) { return false; },
                    _ => return false
                }
                match motion {
                    -7 | 9 | 1 => if !(x2 > x1) { return false; },
                    7 | -9 | -1 => if !(x2 < x1) { return false; },
                    _ => return false
                }
                return true;
            }
        }

        return true;
    }

    fn is_pawn_move_valid(&self, m: &Move, is_white: bool) -> bool {
        // Direction of the move must be valid 
        if (is_white && (m.from / 8) > (m.to / 8)) || (!is_white && (m.from / 8) < (m.to / 8)) {
            return false;
        }

        // Two squares up on the first line
        if m.to == m.from + 16 && m.from / 8 != 1 {
            return false;
        } else if m.to + 16 == m.from && m.from / 8 != 6 {
            return false;
        }

        // If it is a capture, then it has to be diagonal
        if self.has_piece_at(m.to) {
            return m.from % 8 != m.to % 8;
        } else if m.from % 8 != m.to % 8 {
            // diagonal moves need to be captured only moves
            return false;
        }

        // If you end up here, it means that the pawn move is valid
        return true;
    }

    fn is_king_move_valid(&self, m: &Move, is_white: bool) -> bool {
        // First, check if it is one of the casling move.
        let motion = m.to - m.from;
        if motion == 2 || motion == -2 {
            // 1. check that the king did not move or castled
            // using the flag for white or black
            if (is_white && is_set(self.flags, FLAG_WK_MOVED)) || (!is_white && is_set(self.flags, FLAG_BK_MOVED)) {
                return false;
            }

            // check that there is a rook
            if motion > 0 {
                if !is_set(self.rooks, m.from + 3) {
                    return false;
                }
            } else {
                if !is_set(self.rooks, m.from - 4) {
                    return false;
                }
            }
        }
        return true;
    }

    /// Returns true if the move respect the rules of check
    /// This function eventually edits the `quality` property of a move
    fn is_move_valid(&self, mut m: &mut Move) -> bool {
        // Check that there is a piece to move at the destination
        if let Some(t) = self.type_at_index(m.from) {
            let is_white = is_set(self.whites, m.from);
            // If it's a pawn move, check that it is a valid one
            if t == Type::Pawn && !self.is_pawn_move_valid(&m, is_white) {
                return false;
            }

            // If it's a king move, check that it is a valid one
            if t == Type::King && !self.is_king_move_valid(&m, is_white) {
                return false;
            }

            // Check that the destination is valid and that the moves remains within the chess board
            if !self.valid_destination(&mut m) || !self.is_in_bound(&m, &t) {
                return false;
            }

            // Check if the move is not going over another piece
            if !(t == Type::Knight || t == Type::King || self.is_move_over_free_squares(&m)) {
                return false;
            }

            // If we reach this point, it mean the move is valid
            return true;
        }

        return false;
    }

    /// Apply the move without any kind of safety check
    pub fn apply_move_unsafe(&mut self, m: &Move) {
        if let Some(t) = self.type_at_index(m.from) {

            // Eventually apply the capture
            self.apply_capture(&m);

            let is_white = is_set(self.whites, m.from);

            // Apply the move
            match t {
                Type::Pawn => {
                    clear_at!(self.pawns, m.from);
                    // handle the promotion directly here
                    if m.to / 8 == 7 || m.to / 8 == 0 {
                        set_at!(self.queens, m.to);
                    } else {
                        set_at!(self.pawns, m.to);
                    }
                }
                Type::Bishop => {
                    clear_at!(self.bishops, m.from);
                    set_at!(self.bishops, m.to);
                }
                Type::Knight => {
                    clear_at!(self.knights, m.from);
                    set_at!(self.knights, m.to);
                }
                Type::Rook => {
                    clear_at!(self.rooks, m.from);
                    set_at!(self.rooks, m.to);
                }
                Type::Queen => {
                    clear_at!(self.queens, m.from);
                    set_at!(self.queens, m.to);
                }
                Type::King => {
                    clear_at!(self.kings, m.from);
                    set_at!(self.kings, m.to);


                    // Flags update : king moved
                    // TODO OPT: check if setting the flag only at the first is worth it 
                    if is_white {
                        set_at!(self.flags, FLAG_WK_MOVED);
                    } else {
                        set_at!(self.flags, FLAG_BK_MOVED);
                    }

                    // Handle castling move here
                    let motion = m.to - m.from;
                    if motion == 2 || motion == -2 {
                        if is_white {
                            set_at!(self.flags, FLAG_WK_CASTLED);
                        } else {
                            set_at!(self.flags, FLAG_BK_CASTLED);
                        }
                        let (rook_from, rook_to) = if motion == 2 {
                            (m.from + 3, m.from + 1)
                        } else {
                            (m.from - 4, m.from - 1)
                        };
                        // apply the move rook 
                        clear_at!(self.rooks, rook_from);
                        set_at!(self.rooks, rook_to);
                        // apply the change of colors for the rook
                        if is_white {
                            clear_at!(self.whites, rook_from);
                            set_at!(self.whites, rook_to);
                        }
                    }
                }
            }

            // Apply the color to the new square
            if is_white {
                clear_at!(self.whites, m.from);
                set_at!(self.whites, m.to);
            }
        }
    }

    pub fn apply_move_safe(&mut self, mut m: Move) -> bool {
        if self.is_move_valid(&mut m) {
            self.apply_move_unsafe(&m);
            return true;
        }
        return false;
    }

    fn _match_template(&self) {
        let at: i8 = 5;
        if let Some(t) = self.type_at_index(at) {
            match t {
                Type::Pawn => {}
                Type::Bishop => {}
                Type::Knight => {}
                Type::Rook => {}
                Type::Queen => {}
                Type::King => {}
            }
        }
    }

    pub fn new_test1() -> Self {
        let mut whites = 0;
        let mut pawns = 0;
        let mut bishops = 0;
        let knights = 0;
        let mut rooks = 0;
        let queens = 0;
        let kings = 0;

        // Set some pieces here
        set_at!(rooks, pos_to_index(3,3));
        // set_at!(bishops, pos_to_index(3,5));

        set_at!(rooks, pos_to_index(1,3));
        set_at!(whites, pos_to_index(1,3));
        set_at!(pawns, pos_to_index(3,1));
        set_at!(whites, pos_to_index(3,1));

        return Self {
            whites,
            pawns,
            bishops,
            knights,
            rooks,
            queens,
            kings,
            flags: 0,
        };
    }
}

// Functions for the solver

impl ChessGame {
    fn fill_moves(&self, to_fill: &mut Vec<Move>, starting_from: &[i8], motions: &[i8]) {
        for pos in starting_from {
            for motion in motions {
                let des: i8 = pos + motion;
                if des >= 0 && des < 64 {
                    let mut m = Move::new(*pos, des);
                    if self.is_move_valid(&mut m) {
                        to_fill.push(m);
                    }
                }
            }
        }
    }

    pub fn fill_possible_moves_from(&self, mut moves: &mut Vec<Move>, from: i8) -> Vec<i8> {
        if let Some(t) = self.type_at_index(from) {
            let positions: [i8; 1] = [from];
            match t {
                Type::Pawn => self.fill_moves(&mut moves, &positions, &PAWN_MOVES),
                Type::Bishop => self.fill_moves(&mut moves, &positions, &BISHOP_MOVES),
                Type::Rook => self.fill_moves(&mut moves, &positions, &ROOK_MOVES),
                Type::Knight => self.fill_moves(&mut moves, &positions, &KNIGHT_MOVES),
                Type::King => {
                    self.fill_moves(&mut moves, &positions, &KING_MOVES);
                    self.fill_moves(&mut moves, &positions, &KING_SPECIAL_MOVES);
                }
                Type::Queen => self.fill_moves(&mut moves, &positions, &QUEEN_MOVES),
            }
        }

        let attacked = moves.iter().map(|m| m.to).collect();
        return attacked;
    }


    /**
     * Returns the list of possible moves for the requested player.
     */
    pub fn get_avalaible_moves(&self, is_white_playing: bool) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        // all pieces 
        let mut pieces = self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings;

        // saved by color
        if is_white_playing {
            pieces &= self.whites;
        } else {
            pieces &= !self.whites;
        }

        // fill possibles moves from each of the pieces taken in consideration.
        for pos in find_ones(pieces) {
            self.fill_possible_moves_from(&mut moves, pos);
        }

        return moves;
    }

    pub fn score(&self) -> ScoreType {
        let mut score = 0;

        // Value of pieces
        for i in 0..64 {
            if let Some(t) = self.type_at_index(i) {
                if is_set(self.whites, i) {
                    score += t.score();
                } else {
                    score -= t.score();
                }
            }
        }
        // The bigger this ratio is, the less the engine will favor attacking positions.
        score *= 50;

        // Castling : we want to favor the castle, which secures the king
        if is_set(self.flags, FLAG_WK_CASTLED) {
            score += 30;
        }
        if is_set(self.flags, FLAG_BK_CASTLED) {
            score -= 30;
        }

        // TODO: This is really the problem: the number of attacked squres takes a lot of time to be found 
        // and reduces the performs by a factor of 28. Is there a better way to do this ? 

        // Number of attacked squares: this is added to favor an attacking position
        score += self.get_avalaible_moves(true).len() as ScoreType;
        score -= self.get_avalaible_moves(false).len() as ScoreType;

        return score;
    }

    pub fn debug(&self) {
        println!("whites: {} -> {:?}", self.whites, find_ones(self.whites));
        println!("pawns: {} -> {:?}", self.pawns, find_ones(self.pawns));
        println!("rooks: {} -> {:?}", self.rooks, find_ones(self.rooks));
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::hash::Hash;
    use crate::model::game::{ChessGame, ScoreType};
    use crate::model::moves::Move;

    #[test]
    fn test_wrong_knigt_move() {
        let game = ChessGame::new();
        let mut invalid_move = Move::new(6, 31);
        assert!(!game.is_move_valid(&mut invalid_move))
    }
}
