/// Defines chess attacks
mod attacks;

mod constructor;
mod display;
/// Computes some bitmask that can be reused efficently at runtime.
mod precomputation;
mod moves;

use super::moves::*;
use crate::chess_type::Type::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::chess_type::{ScoreType, Type};
use crate::game::attacks::ChessAttacks;
use crate::moves_container::{MovesContainer, SimpleMovesContainer};
use crate::utils::{clear_at, is_set, pos_to_index, set_at, ChessPosition, IntoChessPosition};

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

impl Default for ChessGame {
    fn default() -> Self {
        ChessGame::standard_game()
    }
}

const FLAG_WK_MOVED: i8 = 0;
const FLAG_BK_MOVED: i8 = 1;
const FLAG_WK_CASTLED: i8 = 2;
const FLAG_BK_CASTLED: i8 = 3;

impl ChessGame {
    /// Construct a chess game from the integers
    #[allow(dead_code)]
    pub fn new(
        whites: u64,
        pawns: u64,
        bishops: u64,
        knights: u64,
        rooks: u64,
        queens: u64,
        kings: u64,
        flags: u64,
    ) -> Self {
        Self {
            whites,
            pawns,
            bishops,
            knights,
            rooks,
            queens,
            kings,
            flags,
        }
    }

    /// Returns the type of the provided index.
    /// If no type is present, returns None.
    pub fn type_at_index(&self, at: ChessPosition) -> Option<Type> {
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
    pub(crate) fn has_piece_at(&self, at: ChessPosition) -> bool {
        // TODO maybe adding 1 integer in the struct that just keeps the position of the pieces would be faster than this...
        //      It's a small change to test.
        // Note: since the occupancy grid is computed in many places in the `attacks.rs` module, I really think that
        // this would make sense...
        is_set!(
            self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings,
            at
        )
    }

    /// Adds a piece to self.
    /// Can be used to create custom boards.
    #[allow(dead_code)]
    pub fn set_piece(&mut self, piece: Type, white: bool, at: impl IntoChessPosition) {
        let at: ChessPosition = at.into_position();
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

    #[allow(dead_code)]
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
        let mut current_pos = m.from;
        current_pos += direction;
        while current_pos != m.to {
            if self.has_piece_at(current_pos) {
                return false;
            }
            current_pos += direction;
        }
        true
    }

    /// Asserts that the pawn moves respect some basic rules
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
        let motion = m.to - m.from;
        motion == 8 || motion == -8 || motion == -16 || motion == 16
    }

    fn is_king_move_valid(&self, m: &Move) -> bool {
        // First, check if it is one of the casling move.
        let motion = m.to - m.from;

        if motion == 2 || motion == -2 {
            // 1. check that the king did not move or castled
            // using the flag for white or black
            if (m.is_white && is_set!(self.flags, FLAG_WK_MOVED))
                || (!m.is_white && is_set!(self.flags, FLAG_BK_MOVED))
            {
                return false;
            }

            // Check that there is a rook in the correct position
            if motion > 0 {
                if !is_set!(self.rooks, m.from + 3) {
                    return false;
                }
            } else {
                if !is_set!(self.rooks, m.from - 4) {
                    return false;
                }
            }

            // Check that the neighbors positions are empty and not attacked
            let positions_to_check = if motion > 0 {
                vec![m.from + 1, m.from + 2]
            } else {
                vec![m.from - 1, m.from - 2, m.from - 3]
            };

            // First check the occupancy, as it cost less
            let occupancy =
                self.rooks | self.kings | self.queens | self.pawns | self.bishops | self.knights;
            for pos in &positions_to_check {
                if is_set!(occupancy, pos) {
                    return false;
                }
            }

            // Second check the attacks
            let attacked = self.get_attacked_squares(!m.is_white);
            for pos in positions_to_check {
                if is_set!(attacked, pos) {
                    return false;
                }
            }

            return true;
        }

        // Otherwise, only a set of moves are accepted
        motion == 1
            || motion == -1
            || motion == 8
            || motion == -8
            || motion == 7
            || motion == -7
            || motion == 9
            || motion == -9
    }

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
            };
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

    /// Returns true if the destination
    fn is_destination_of_incorrect_color(&self, m: &Move) -> bool {
        self.has_piece_at(m.to) && is_set!(self.whites, m.to) == is_set!(self.whites, m.from)
    }

    pub(crate) fn is_move_valid_for_type(&self, m: &Move, t: Type) -> bool {
        // In the case where there is a piece at the last position, check that it has a different color
        if self.is_destination_of_incorrect_color(&m) {
            return false;
        }

        // Check if the move is not going over another piece
        if !(t == Knight || t == King || self.is_move_over_free_squares(&m)) {
            return false;
        }

        let motion = m.to - m.from;
        let (x1, y1, x2, y2) = (m.from % 8, m.from / 8, m.to % 8, m.to / 8);

        match t {
            Pawn => self.is_pawn_move_valid(&m),
            Bishop => Self::is_bishop_valid(&motion, &x1, &y1, &x2, &y2),
            Rook => Self::is_rook_valid(&motion, &m),
            Queen => {
                Self::is_bishop_valid(&motion, &x1, &y1, &x2, &y2)
                    || Self::is_rook_valid(&motion, &m)
            }
            Knight => match motion {
                17 | 10 => x2 > x1 && y2 > y1,
                15 | 6 => x2 < x1 && y2 > y1,
                -10 | -17 => x2 < x1 && y2 < y1,
                -15 | -6 => x2 > x1 && y2 < y1,
                _ => false,
            },
            King => self.is_king_move_valid(&m),
        }
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

    /// Returns true if the move respect the rules of check
    /// This function eventually edits the `quality` property of a move
    fn is_move_valid(&self, m: &Move) -> bool {
        self.type_at_index(m.from)
            .map(|t| self.is_move_valid_for_type(m, t))
            .unwrap_or(false)
    }

    /// Applies a move after checking all the rules
    /// This function is never called in the optimisation
    pub fn apply_move_safe(&mut self, m: Move) -> bool {
        if self.is_move_valid(&m) {
            println!("Move: {m:?} is valid");
            self.apply_move_unsafe(&m);
            return true;
        }
        false
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

    #[allow(dead_code)]
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
        println!(
            "({}, {}, {}, {}, {}, {}, {}, {})",
            self.whites,
            self.pawns,
            self.bishops,
            self.knights,
            self.rooks,
            self.queens,
            self.kings,
            self.flags
        );
        println!("----");
    }

    /// Returns the type of the provided position.
    /// If no type is present, returns None.
    #[allow(dead_code)]
    pub fn type_at_xy(&self, x: i8, y: i8) -> Option<Type> {
        self.type_at_index(pos_to_index(x, y))
    }

    /// Returns true if the given (x,y) coordinates contains a white piece
    #[allow(dead_code)]
    pub fn is_white_at_xy(&self, x: i8, y: i8) -> bool {
        is_set!(self.whites, pos_to_index(x, y))
    }

    #[allow(dead_code)]
    pub fn is_black_at(&self, pos: ChessPosition) -> bool {
        self.type_at_index(pos).is_some() && !is_set!(self.whites, pos)
    }
}

#[cfg(test)]
mod tests {
    use crate::chess_type::Type::Pawn;
    use crate::game::ChessGame;
    use crate::moves::Move;

    #[test]
    fn test_wrong_knight_move() {
        let game = ChessGame::standard_game();
        let mut invalid_move = Move::new(6, 31, true);
        assert!(!game.is_move_valid(&mut invalid_move))
    }

    #[test]
    fn test_small_castle() {
        // This is the chess position reached after
        // 1. e4 e5
        // 2. Nf3 Qf6
        // 3. Bc4 Qf4
        // which by the way is terrible for black...
        // It is white's turn
        // White can castle or move the king up
        let mut game = ChessGame {
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
        assert!(game.is_move_valid(&mut move1));

        // but if we block castling, the move is not valid
        game.block_castling();
        assert!(!game.is_move_valid(&mut move1));
    }

    #[test]
    fn test_invalid_small_castle() {
        // GIVEN
        // (e4, _)
        // (Nf3, _)
        let game = ChessGame {
            whites: 270593983,
            pawns: 65038346434440960,
            bishops: 2594073385365405732,
            knights: 4755801206505340930,
            rooks: 9295429630892703873,
            queens: 576460752303423496,
            kings: 1152921504606846992,
            flags: 0,
        };

        // THEN white must not be able to castle
        let mut move1 = Move::new(4, 6, true);
        assert!(!game.is_move_valid(&mut move1));
    }

    #[test]
    fn test_simple_motions() {
        // Create a new chess game
        let mut game = ChessGame::standard_game();

        // A stupid move should not pass
        let m = Move::new(10, 11, false);
        let mut result = game.apply_move_safe(m);
        assert_eq!(result, false);

        // e4 is a valid move
        result = game.apply_move_safe(Move::new(12, 28, true));
        assert_eq!(result, true);

        // my chess representation does not keep track of who is the current player
        // e5 for white is a valid move
        result = game.apply_move_safe(Move::new(28, 36, true));
        assert_eq!(result, true);

        // e5 for black is not valid
        result = game.apply_move_safe(Move::new(52, 36, false));
        assert_eq!(result, false);

        // e6 for black is valid
        result = game.apply_move_safe(Move::new(52, 44, false));
        assert_eq!(result, true);
    }

    #[test]
    fn test_king_cant_go_everywhere() {
        // Create a new chess game
        let mut game = ChessGame::standard_game();

        // Try moving the king from e1 to e4
        let m = Move::new(4, 28, true);
        let result = game.apply_move_safe(m);

        // Assert that the move is not allowed
        assert_eq!(result, false);
    }

    #[test]
    fn test_initial_score() {
        let game = ChessGame::standard_game();
        assert_eq!(0, game.score());
    }

    #[test]
    fn test_score1() {
        let mut game = ChessGame::empty();
        game.set_piece(Pawn, true, "e2");
        game.set_piece(Pawn, false, "e7");
        assert_eq!(0, game.score());
    }

    #[test]
    fn test_invalid_pawn_move_at_begining() {
        let game = ChessGame::standard_game();
        assert!(!game.is_move_valid(&Move::new(12, 36, true)))
    }

    #[test]
    fn test_gpt_crazy_shit() {
        let mut pieces = 0b01001100u8;
        while pieces != 0 {
            let first_piece = pieces.trailing_zeros();
            println!("{first_piece}");
            pieces &= pieces - 1;
        }
    }
}
