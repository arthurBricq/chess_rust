use crate::chess_type::Type;
use crate::chess_type::Type::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::game::attacks::ChessAttacks;
use crate::game::ChessGame;
use crate::motion_iterator::StepMotionIterator;
use crate::moves::{
    Move, BLACK_PAWN_MOVES, KING_MOVES, KING_SPECIAL_MOVES, KNIGHT_MOVES, WHITE_PAWN_MOVES,
};
use crate::moves_container::MovesContainer;
use crate::utils::ChessPosition;

impl ChessGame {
    /// Fills the provided container with all the available moves at the current position.
    ///
    /// This function also resets the move container before running anything.
    ///
    /// TODO see how I could use bitmasks to improve this computation
    /// TODO would be nice to move this function (and associated helper) in a submodule
    pub fn update_move_container<T: MovesContainer>(&self, container: &mut T, is_white: bool) {
        container.reset();

        // """"""""""
        // 1. New way
        // """"""""""

        let mut attacks = self.get_attacked_squares_knight(is_white);
        while attacks != 0 {
            let sq = attacks.trailing_zeros() as usize;
            
            
            attacks &= attacks - 1;
        }
        
        // Consume all the attacks

        // """"""""""
        // 2. Old Way
        // """"""""""

        let mut pieces = if is_white {
            (self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings)
                & self.whites
        } else {
            (self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings)
                & !self.whites
        };

        while pieces != 0 {
            let i = pieces.trailing_zeros() as ChessPosition;
            pieces &= pieces - 1;
            match self.type_at_index(i).unwrap() {
                Pawn => {
                    if is_white {
                        self.fill_move_container_with_list_of_moves(
                            container,
                            i,
                            &WHITE_PAWN_MOVES,
                            is_white,
                            Pawn,
                        );
                    } else {
                        self.fill_move_container_with_list_of_moves(
                            container,
                            i,
                            &BLACK_PAWN_MOVES,
                            is_white,
                            Pawn,
                        );
                    }
                }
                Knight => {}
                King => {
                    self.fill_move_container_with_list_of_moves(
                        container,
                        i,
                        &KING_MOVES,
                        is_white,
                        King,
                    );
                    self.fill_move_container_with_list_of_moves(
                        container,
                        i,
                        &KING_SPECIAL_MOVES,
                        is_white,
                        King,
                    );
                }
                Bishop => self.fill_move_container_with_iterator(
                    container,
                    &mut [
                        StepMotionIterator::new(i, 9, is_white, Bishop),
                        StepMotionIterator::new(i, -9, is_white, Bishop),
                        StepMotionIterator::new(i, 7, is_white, Bishop),
                        StepMotionIterator::new(i, -7, is_white, Bishop),
                    ],
                ),
                Rook => self.fill_move_container_with_iterator(
                    container,
                    &mut [
                        StepMotionIterator::new(i, 1, is_white, Rook),
                        StepMotionIterator::new(i, -1, is_white, Rook),
                        StepMotionIterator::new(i, 8, is_white, Rook),
                        StepMotionIterator::new(i, -8, is_white, Rook),
                    ],
                ),
                Queen => {
                    self.fill_move_container_with_iterator(
                        container,
                        &mut [
                            StepMotionIterator::new(i, 9, is_white, Queen),
                            StepMotionIterator::new(i, -9, is_white, Queen),
                            StepMotionIterator::new(i, 7, is_white, Queen),
                            StepMotionIterator::new(i, -7, is_white, Queen),
                        ],
                    );
                    self.fill_move_container_with_iterator(
                        container,
                        &mut [
                            StepMotionIterator::new(i, 1, is_white, Queen),
                            StepMotionIterator::new(i, -1, is_white, Queen),
                            StepMotionIterator::new(i, 8, is_white, Queen),
                            StepMotionIterator::new(i, -8, is_white, Queen),
                        ],
                    );
                }
            }
        }
    }

    /// Push valid moves in the `MovesContainer`, by trying all the possible moves given
    /// in `motions`
    fn fill_move_container_with_list_of_moves(
        &self,
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

    /// Push valid moves in the `MovesContainer`, while going in the direction of the motion
    /// iterator.
    fn fill_move_container_with_iterator(
        &self,
        to_fill: &mut dyn MovesContainer,
        iterators: &mut [StepMotionIterator],
    ) {
        for iter in iterators {
            while let Some(m) = iter.next(self) {
                to_fill.push(m)
            }
        }
    }
}
