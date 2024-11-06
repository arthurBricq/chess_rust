use super::moves::*;
use crate::model::chess_type::Type::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::model::chess_type::{ScoreType, Type};
use crate::model::motion_iterator::StepMotionIterator;
use crate::model::moves_container::{MovesContainer, SimpleMovesContainer};
use crate::model::tools::{clear_at, is_set, pos_to_index, set_at};

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
    pub(crate) whites: u64,
    pub(crate) pawns: u64,
    pub(crate) bishops: u64,
    pub(crate) knights: u64,
    pub(crate) rooks: u64,
    pub(crate) queens: u64,
    pub(crate) kings: u64,
    pub(crate) flags: u64,
}

const FLAG_WK_MOVED: i8 = 0;
const FLAG_BK_MOVED: i8 = 1;
const FLAG_WK_CASTLED: i8 = 2;
const FLAG_BK_CASTLED: i8 = 3;


impl ChessGame {
    /// Construct a chess game from the integers
    #[cfg(test)]
    pub fn new(whites: u64, pawns: u64, bishops: u64, knights: u64, rooks: u64, queens: u64, kings: u64, flags: u64) -> Self {
        Self { whites, pawns, bishops, knights, rooks, queens, kings, flags }
    }

    /// Returns the type of the provided position.
    /// If no type is present, returns None.
    pub fn type_at(&self, x: i8, y: i8) -> Option<Type> {
        self.type_at_index(pos_to_index(x, y))
    }

    /// Returns true if the given (x,y) coordinates contains a white piece
    pub fn is_white_at(&self, x: i8, y: i8) -> bool {
        is_set!(self.whites, pos_to_index(x, y))
    }

    /// Returns the type of the provided index. 
    /// If no type is present, returns None.
    pub fn type_at_index(&self, at: i8) -> Option<Type> {
        if is_set!(self.pawns, at) {
            Some(Pawn)
        } else if is_set!(self.bishops, at) {
            Some(Bishop)
        } else if is_set!(self.knights, at) {
            Some(Knight)
        } else if is_set!(self.rooks, at) {
            Some(Rook)
        } else if is_set!(self.kings, at) {
            Some(King)
        } else if is_set!(self.queens, at) {
            Some(Queen)
        } else {
            None
        }
    }

    /// Returns true if there is a piece at this position
    pub(crate) fn has_piece_at(&self, at: i8) -> bool {
        is_set!(self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings, at)
    }

    /// Adds a piece to self.
    /// Can be used to create custom boards.
    pub fn set_piece(&mut self, piece: Type, white: bool, at: u8) {
        match piece {
            Pawn => set_at!(self.pawns, at),
            Bishop => set_at!(self.bishops, at),
            Knight => set_at!(self.knights, at),
            Rook => set_at!(self.rooks, at),
            Queen => set_at!(self.queens, at),
            King => set_at!(self.kings, at),
        }
        if white {
            set_at!(self.whites, at)
        }
    }

    pub fn block_castling(&mut self) {
        set_at!(self.flags, FLAG_BK_CASTLED);
        set_at!(self.flags, FLAG_WK_CASTLED);
        set_at!(self.flags, FLAG_WK_MOVED);
        set_at!(self.flags, FLAG_BK_MOVED);
    }

    /// Returns true if one of the two kind is dead
    pub fn is_finished(&self) -> bool {
        self.kings.count_ones() != 2
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
        true
    }

    /// Returns true if the destination is in bound
    pub(crate) fn is_in_bound(&self, m: &Move, t: &Type) -> bool {
        let motion = m.to - m.from;
        let (x1, y1, x2, y2) = (m.from % 8, m.from / 8, m.to % 8, m.to / 8);

        fn is_bishop_valid(motion: &i8, x1: &i8, y1: &i8, x2: &i8, y2: &i8) -> bool {
            if motion % 7 == 0 {
                return if *motion > 0 {
                    // upper left diagonal
                    x1 > x2 && y1 < y2
                } else {
                    // lower right diagonal
                    x1 < x2 && y1 > y2
                };
            } else if motion % 9 == 0 {
                return if *motion > 0 {
                    // upper right diagonal
                    x1 < x2 && y1 < y2
                } else {
                    // lower left diagonal
                    x1 > x2 && y1 > y2
                }
            }
            false
        }

        fn is_rook_valid(motion: &i8, m: &Move) -> bool {
            // Look on the same line
            let remain = m.from % 8;
            let min = m.from - remain;
            let max = m.from + (7 - remain);
            if m.to >= min && m.to <= max {
                true
            } else {
                // Then look up and down
                motion % 8 == 0
            }
        }

        match t {
            Pawn => {
                (x2 - x1).abs() <= 1
            }
            Bishop => {
                is_bishop_valid(&motion, &x1, &y1, &x2, &y2)
            }
            Rook => {
                is_rook_valid(&motion, &m)
            }
            Queen => {
                is_bishop_valid(&motion, &x1, &y1, &x2, &y2) || is_rook_valid(&motion, &m)
            }
            Knight => {
                match motion {
                    17 | 10 => x2 > x1 && y2 > y1,
                    15 | 6 => x2 < x1 && y2 > y1,
                    -10 | -17 => x2 < x1 && y2 < y1,
                    -15 | -6 => x2 > x1 && y2 < y1,
                    _ => false
                }
            }
            King => {
                match motion {
                    7 | 8 | 9 => if !(y2 > y1) { return false; },
                    -7 | -8 | -9 => if !(y2 < y1) { return false; },
                    _ => {}
                }
                match motion {
                    -7 | 9 | 1 => if !(x2 > x1) { return false; },
                    7 | -9 | -1 => if !(x2 < x1) { return false; },
                    _ => {}
                }
                true
            }
        }
    }

    fn is_pawn_move_valid(&self, m: &Move) -> bool {
        // Direction of the move must be valid 
        if (m.is_white && (m.from / 8) > (m.to / 8)) || (!m.is_white && (m.from / 8) < (m.to / 8)) {
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
        true
    }

    fn is_king_move_valid(&self, m: &Move) -> bool {
        // First, check if it is one of the casling move.
        let motion = m.to - m.from;
        if motion == 2 || motion == -2 {
            // 1. check that the king did not move or castled
            // using the flag for white or black
            if (m.is_white && is_set!(self.flags, FLAG_WK_MOVED)) || (!m.is_white && is_set!(self.flags, FLAG_BK_MOVED)) {
                return false;
            }

            // check that there is a rook
            if motion > 0 {
                if !is_set!(self.rooks, m.from + 3) {
                    return false;
                }
            } else {
                if !is_set!(self.rooks, m.from - 4) {
                    return false;
                }
            }
        }

        // Otherwise, only a set of moves are accepted
        motion == 1 || motion == -1
            || motion == 8 || motion == -8
            || motion == 7 || motion == -7
            || motion == 9 || motion == -9
    }

    fn is_move_valid_for_type(&self, m: &Move, t: Type) -> bool {
        if match t {
            Pawn => !self.is_pawn_move_valid(&m),
            King => !self.is_king_move_valid(&m),
            _ => false
        } {
            return false;
        }

        // In the case where there is a piece at the last position, check that it has a different color
        if self.is_destination_of_incorrect_color(&m) {
            return false;
        }

        // Check that the destination is valid and that the moves remains within the chess board
        // TODO when calling from `score`, we know that the rules are respected
        if !self.is_in_bound(&m, &t) {
            return false;
        }

        // Check if the move is not going over another piece
        if !(t == Knight || t == King || self.is_move_over_free_squares(&m)) {
            return false;
        }

        true
    }

    /// Returns true if the move respect the rules of check
    /// This function eventually edits the `quality` property of a move
    fn is_move_valid(&self, m: &Move) -> bool {
        self.type_at_index(m.from)
            .map(|t| self.is_move_valid_for_type(m, t))
            .unwrap_or(false)
    }

    /// Returns true if the destination
    pub(crate) fn is_destination_of_incorrect_color(&self, m: &Move) -> bool {
        self.has_piece_at(m.to) && is_set!(self.whites, m.to) == is_set!(self.whites, m.from)
    }

    /// Apply the move without any kind of safety check
    pub fn apply_move_unsafe(&mut self, m: &Move) {
        if let Some(t) = self.type_at_index(m.from) {

            // Eventually apply the capture
            self.apply_capture(&m);

            // Apply the move
            match t {
                Pawn => {
                    clear_at!(self.pawns, m.from);
                    // handle the promotion directly here
                    if m.to / 8 == 7 || m.to / 8 == 0 {
                        set_at!(self.queens, m.to);
                    } else {
                        set_at!(self.pawns, m.to);
                    }
                }
                Bishop => {
                    clear_at!(self.bishops, m.from);
                    set_at!(self.bishops, m.to);
                }
                Knight => {
                    clear_at!(self.knights, m.from);
                    set_at!(self.knights, m.to);
                }
                Rook => {
                    clear_at!(self.rooks, m.from);
                    set_at!(self.rooks, m.to);
                }
                Queen => {
                    clear_at!(self.queens, m.from);
                    set_at!(self.queens, m.to);
                }
                King => {
                    clear_at!(self.kings, m.from);
                    set_at!(self.kings, m.to);

                    // Flags update : king moved
                    // TODO OPT: check if setting the flag only at the first is worth it 
                    if m.is_white {
                        set_at!(self.flags, FLAG_WK_MOVED);
                    } else {
                        set_at!(self.flags, FLAG_BK_MOVED);
                    }

                    // Handle castling move here
                    let motion = m.to - m.from;
                    if motion == 2 || motion == -2 {
                        if m.is_white {
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
                        if m.is_white {
                            clear_at!(self.whites, rook_from);
                            set_at!(self.whites, rook_to);
                        }
                    }
                }
            }

            // Apply the color to the new square
            if m.is_white {
                clear_at!(self.whites, m.from);
                set_at!(self.whites, m.to);
            }
        }
    }

    pub fn apply_move_safe(&mut self, m: Move) -> bool {
        if self.is_move_valid(&m) {
            self.apply_move_unsafe(&m);
            return true;
        }
        false
    }

    /// Push valid moves in the `MovesContainer`, while going in the direction of the motion
    /// iterator.
    fn fill_move_container_with_iterator(&self, to_fill: &mut dyn MovesContainer, iterators: &mut [StepMotionIterator]) {
        for iter in iterators {
            while let Some(m) = iter.next(self) {
                to_fill.push(m)
            }
        }
    }

    /// Push valid moves in the `MovesContainer`, by trying all the possible moves given
    /// in `motions`
    fn fill_move_container_with_list_of_moves(&self,
                                              to_fill: &mut dyn MovesContainer,
                                              from: i8,
                                              motions: &[i8],
                                              is_white: bool,
                                              t: Type,
    ) {
        for motion in motions {
            let des: i8 = from + motion;
            if des >= 0 && des < 64 {
                let mut m = Move::new(from, des, is_white);
                if self.is_move_valid_for_type(&m, t) {
                    if let Some(captured) = self.type_at_index(m.to) {
                        let piece = self.type_at_index(m.from).unwrap();
                        m.set_quality_from_scores(piece.score(), captured.score());
                    }
                    to_fill.push(m);
                }
            }
        }
    }

    /// Fills the provided container with all the available moves at the current position.
    ///
    /// This function also resets the move container before running anything.
    pub fn update_move_container<T: MovesContainer>(&self, container: &mut T, is_white: bool) {
        container.reset();

        let pieces = if is_white {
            (self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings) & self.whites
        } else {
            (self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings) & !self.whites
        };

        for i in 0..64 {
            if !is_set!(pieces, i) {
                continue;
            }

            match self.type_at_index(i).unwrap() {
                Pawn => {
                    if is_white {
                        self.fill_move_container_with_list_of_moves(container, i, &WHITE_PAWN_MOVES, is_white, Pawn);
                    } else {
                        self.fill_move_container_with_list_of_moves(container, i, &BLACK_PAWN_MOVES, is_white, Pawn);
                    }
                }
                Knight => {
                    self.fill_move_container_with_list_of_moves(container, i, &KNIGHT_MOVES, is_white, Knight)
                }
                King => {
                    self.fill_move_container_with_list_of_moves(container, i, &KING_MOVES, is_white, King);
                    self.fill_move_container_with_list_of_moves(container, i, &KING_SPECIAL_MOVES, is_white, King);
                }
                Bishop => {
                    self.fill_move_container_with_iterator(container, &mut [
                        StepMotionIterator::new(i, 9, is_white, Bishop),
                        StepMotionIterator::new(i, -9, is_white, Bishop),
                        StepMotionIterator::new(i, 7, is_white, Bishop),
                        StepMotionIterator::new(i, -7, is_white, Bishop),
                    ])
                }
                Rook => {
                    self.fill_move_container_with_iterator(container, &mut [
                        StepMotionIterator::new(i, 1, is_white, Rook),
                        StepMotionIterator::new(i, -1, is_white, Rook),
                        StepMotionIterator::new(i, 8, is_white, Rook),
                        StepMotionIterator::new(i, -8, is_white, Rook),
                    ])
                }
                Queen => {
                    self.fill_move_container_with_iterator(container, &mut [
                        StepMotionIterator::new(i, 9, is_white, Queen),
                        StepMotionIterator::new(i, -9, is_white, Queen),
                        StepMotionIterator::new(i, 7, is_white, Queen),
                        StepMotionIterator::new(i, -7, is_white, Queen),
                    ]);
                    self.fill_move_container_with_iterator(container, &mut [
                        StepMotionIterator::new(i, 1, is_white, Queen),
                        StepMotionIterator::new(i, -1, is_white, Queen),
                        StepMotionIterator::new(i, 8, is_white, Queen),
                        StepMotionIterator::new(i, -8, is_white, Queen),
                    ]);
                }
            }
        }
    }

    pub fn score(&self) -> ScoreType {
        let mut score = 0;

        score += (self.pawns & self.whites).count_ones() as ScoreType * 1;
        score += ((self.bishops | self.knights) & self.whites).count_ones() as ScoreType * 3;
        score += (self.rooks & self.whites).count_ones() as ScoreType * 5;
        score += (self.queens & self.whites).count_ones() as ScoreType * 10;
        score += (self.kings & self.whites).count_ones() as ScoreType * 1000;

        score -= ((self.bishops | self.knights) & !self.whites).count_ones() as ScoreType * 3;
        score -= (self.pawns & !self.whites).count_ones() as ScoreType * 1;
        score -= (self.rooks & !self.whites).count_ones() as ScoreType * 5;
        score -= (self.queens & !self.whites).count_ones() as ScoreType * 10;
        score -= (self.kings & !self.whites).count_ones() as ScoreType * 1000;

        // Castling : we want to favor the castle, which secures the king
        // if is_set!(self.flags, FLAG_WK_CASTLED) { score += 3; }
        // if is_set!(self.flags, FLAG_BK_CASTLED) { score -= 3; }

        // This is really the problem: the number of attacked squres takes a lot of time to be found
        // and reduces the performs by a factor of 28. Is there a better way to do this ? 

        // Number of attacked squares
        // The bigger this ratio is, the less the engine will favor attacking positions.
        score *= 20;
        let mut container = SimpleMovesContainer::new();
        self.update_move_container(&mut container, true);
        score += container.count() as ScoreType;
        self.update_move_container(&mut container, false);
        score -= container.count() as ScoreType;

        score
    }

    pub fn print_game_integers(&self) {
        println!("\n----");
        println!("whites:  {},", self.whites);
        println!("pawns:   {},", self.pawns);
        println!("bishops: {},", self.bishops);
        println!("knights: {},", self.knights);
        println!("rooks:   {},", self.rooks);
        println!("queens:  {},", self.queens);
        println!("kings:   {},", self.kings);
        println!("flags:   {}", self.flags);
        println!("score = {}", self.score());
        println!("----");
        println!("({}, {}, {}, {}, {}, {}, {}, {})", self.whites, self.pawns, self.bishops, self.knights, self.rooks, self.queens, self.kings, self.flags);
        println!("----");
    }
}

#[cfg(test)]
mod tests {
    use crate::model::game::ChessGame;
    use crate::model::game_constructor::GameConstructor;
    use crate::model::moves::Move;

    #[test]
    fn test_wrong_knigt_move() {
        let game = GameConstructor::standard_game();
        let mut invalid_move = Move::new(6, 31, true);
        assert!(!game.is_move_valid(&mut invalid_move))
    }

    #[test]
    fn test_valid_king_moves() {
        // This is the chess position reached after
        // (e4,e5)
        // (Nf3, Qf6)
        // (Bc4, Qf4)
        // which by the way is terrible for black...
        // It is white's turn
        // White can castle or move the king up
        let game = ChessGame {
            whites: 337702815,
            pawns: 67272588421820160,
            bishops: 2594073385432514564,
            knights: 4755801206505340930,
            rooks: 9295429630892703873,
            queens: 536870920,
            kings: 1152921504606846992,
            flags: 0,
        };

        // this is castle
        // it is valid
        let mut move1 = Move::new(4, 6, true);
        assert!(game.is_move_valid(&mut move1))
    }
}
