use crate::chess_type::Type;
use crate::chess_type::Type::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::game::precomputation::{
    KING_ATTACK_MASKS, KNIGHT_ATTACK_MASKS, PAWN_ATTACK_MASKS, PAWN_MOTION_MASKS,
    SLIDING_ATTACK_MASKS,
};
use crate::game::{ChessGame, FLAG_WHITE_KING_MOVED};
use crate::motion_iterator::StepMotionIterator;
use crate::moves::MoveQuality::{EqualCapture, GoodCapture};
use crate::moves::{
    Move, BLACK_PAWN_MOVES, KING_MOVES, KING_SPECIAL_MOVES, KNIGHT_MOVES, WHITE_PAWN_MOVES,
};
use crate::moves_container::MovesContainer;
use crate::utils::{consume_bits, is_set, pieces_for_color, ChessPosition};
use std::ops::Range;
use crate::game::attacks::ChessAttacks;

impl ChessGame {
    /// Fills the provided container with all the available moves at the current position.
    ///
    /// This function also resets the move container before running anything.
    pub fn update_move_container_old<T: MovesContainer>(&self, container: &mut T, is_white: bool) {
        container.reset();

        // """"""""""
        // 2. Old Way
        // """"""""""

        let pieces = if is_white {
            (self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings)
                & self.whites
        } else {
            (self.pawns | self.bishops | self.knights | self.rooks | self.queens | self.kings)
                & !self.whites
        };

        consume_bits!(pieces, i, {
            let i = i as ChessPosition;
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
                Knight => self.fill_move_container_with_list_of_moves(
                    container,
                    i,
                    &KNIGHT_MOVES,
                    is_white,
                    Knight,
                ),
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
        });
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

impl ChessGame {
    pub fn update_move_container<T: MovesContainer>(&self, container: &mut T, white_playing: bool) {
        container.reset();

        let occupancy =
            self.rooks | self.kings | self.queens | self.pawns | self.bishops | self.knights;

        // 1. Knights

        let pieces = pieces_for_color!(self.whites, self.knights, white_playing);
        consume_bits!(pieces, from, {
            let attacked = KNIGHT_ATTACK_MASKS[from];
            consume_bits!(attacked, to, {
                // TODO This piece of code is copied 3 times...
                let occupied = is_set!(occupancy, to);
                if !occupied {
                    container.push(Move::new(
                        // TODO Can I avoid casting ? Does it have any impact on performance ?
                        from as ChessPosition,
                        to as ChessPosition,
                        white_playing,
                    ));
                } else if is_set!(self.whites, to) != white_playing {
                    // TODO assert if it is worth checking which piece is being captures to have a better ordering
                    let mut m =
                        Move::new(from as ChessPosition, to as ChessPosition, white_playing);
                    m.set_quality(EqualCapture);
                    container.push(m);
                }
            })
        });

        // 2. Kings

        let pieces = pieces_for_color!(self.whites, self.kings, white_playing);
        consume_bits!(pieces, from, {
            let attacked = KING_ATTACK_MASKS[from];
            consume_bits!(attacked, to, {
                let occupied = is_set!(occupancy, to);
                if !occupied {
                    container.push(Move::new(
                        from as ChessPosition,
                        to as ChessPosition,
                        white_playing,
                    ));
                } else if is_set!(self.whites, to) != white_playing {
                    let mut m =
                        Move::new(from as ChessPosition, to as ChessPosition, white_playing);
                    m.set_quality(EqualCapture);
                    container.push(m);
                }
            });
        });

        // 3. Pawns

        // 3.a attacks

        let (white_pawn_attacks, black_pawn_attacks) = &*PAWN_ATTACK_MASKS;
        let attack_masks = if white_playing {
            white_pawn_attacks
        } else {
            black_pawn_attacks
        };

        let pieces = pieces_for_color!(self.whites, self.pawns, white_playing);
        consume_bits!(pieces, from, {
            let attacked = attack_masks[from];
            consume_bits!(attacked, to, {
                // For pawns to move on an attack, the square must be occupied
                let occupied = is_set!(occupancy, to);
                if occupied && (is_set!(self.whites, to) != white_playing) {
                    let mut m = Move::new(
                        from as ChessPosition,
                        to as ChessPosition,
                        white_playing,
                    );
                    m.set_quality(GoodCapture);
                    container.push(m);
                }
            });
        });

        // 3.b Pawns motions

        // TODO Assert whether for pawns, using a mask makes sense at all.
        //      Maybe it is equally fast, or even faster, to just compute manually here.
        //      This is because the second for-loop always contain 1 element, but the compiler
        //      does not know about this.
        let (white_pawn_motion, black_pawn_motions) = &*PAWN_MOTION_MASKS;
        let motion_mask = if white_playing {
            white_pawn_motion
        } else {
            black_pawn_motions
        };

        let pieces = pieces_for_color!(self.whites, self.pawns, white_playing);
        consume_bits!(pieces, from, {
            let possible_moves = motion_mask[from];
            consume_bits!(possible_moves, to, {
                if !is_set!(occupancy, to) {
                    container.push(Move::new(
                        from as ChessPosition,
                        to as ChessPosition,
                        white_playing,
                    ));
                }
            });

            // Pawn special moves: two squares up or down
            let rank = from / 8;
            if white_playing && rank == 1 && !is_set!(occupancy, from + 8) && !is_set!(occupancy, from + 16) {
                container.push(Move::new(
                    from as ChessPosition,
                    from as ChessPosition + 16,
                    white_playing,
                ));
            }

            if !white_playing && rank == 6 && !is_set!(occupancy, from - 8) && !is_set!(occupancy, from - 16) {
                container.push(Move::new(
                    from as ChessPosition,
                    from as ChessPosition - 16,
                    white_playing,
                ));
            }

        });

        // 4. Rooks
        let rook_left = pieces_for_color!(self.whites, self.rooks, white_playing);
        self.fill_attacked_squares_from_sliding_piece(rook_left, 0..4, white_playing, container);

        // 5. Bishops
        let bishops = pieces_for_color!(self.whites, self.bishops, white_playing);
        self.fill_attacked_squares_from_sliding_piece(bishops, 4..8, white_playing, container);

        // 6. Queens
        let queens = pieces_for_color!(self.whites, self.queens, white_playing);
        self.fill_attacked_squares_from_sliding_piece(queens, 0..8, white_playing, container);

        // 7. Castle
        // TODO
        // 1. check that the king did not move or castled
        // using the flag for white or black
        
        /*
        

        // White castling
        if white_playing && !is_set!(self.flags, FLAG_WHITE_KING_MOVED)
        {
            // For white, king is at position 4
            let mut attacked: Option<u64> = None; // `None` means we haven't computed it yet

            // Check occupancy for first condition
            if !is_set!(occupancy, 5) && !is_set!(occupancy, 6) {
                // Compute attacked squares only if needed
                if attacked.is_none() {
                    attacked = Some(self.get_attacked_squares(false));
                }
                if let Some(attacked) = attacked {
                    if !is_set!(attacked, 4) && !is_set!(attacked, 5) && !is_set!(attacked, 6) {
                        // White can small castle
                        container.push(Move::new(4, 6, white_playing));
                    }
                }
            }

            // Check occupancy for second condition
            if !is_set!(occupancy, 3) && !is_set!(occupancy, 2) && !is_set!(occupancy, 1) {
                // Compute attacked squares only if needed (if not already done)
                if attacked.is_none() {
                    attacked = Some(self.get_attacked_squares(false));
                }
                if let Some(attacked) = attacked {
                    if !is_set!(attacked, 4) && !is_set!(attacked, 3) && !is_set!(attacked, 2) {
                        // White can large castle
                        container.push(Move::new(4, 2, white_playing));
                    }
                }
            }

        }
         */



    }

    fn fill_attacked_squares_from_sliding_piece<T: MovesContainer>(
        &self,
        pieces: u64,
        direction_indices: Range<usize>,
        white_playing: bool,
        container: &mut T,
    ) {
        // TODO factorize this
        let occupancy =
            self.rooks | self.kings | self.queens | self.pawns | self.bishops | self.knights;

        consume_bits!(pieces, from, {
            // For each direction
            for dir in direction_indices.clone() {
                // Get the attack ray for this direction from the precomputed sliding masks
                let ray = &SLIDING_ATTACK_MASKS[dir][from];
                // Go through all the positions
                for to in ray {
                    let occupied = is_set!(occupancy, to);
                    if !occupied {
                        container.push(Move::new(from as ChessPosition, *to, white_playing));
                    } else if is_set!(self.whites, to) != white_playing {
                        let mut m = Move::new(from as ChessPosition, *to, white_playing);
                        m.set_quality(EqualCapture);
                        container.push(m);
                    }

                    if occupied {
                        break;
                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::game::ChessGame;
    use crate::moves_container::{MovesContainer, SimpleMovesContainer};

    #[test]
    fn test_blocked_pawns() {
        let mut game = ChessGame::from_fen("4k3/4p3/4n3/8/8/4N3/4P3/4K3 w - - 0 1");
        game.block_castling();
        let mut container = SimpleMovesContainer::new();

        // In this position, white can't move the pawn because it is blocked by the knights.
        // Count of valid positions: 8 for the knight, 4 for the king -> 12
        game.update_move_container(&mut container, true);
        assert_eq!(12, container.count());

        // Same for black
        game.update_move_container(&mut container, false);
        assert_eq!(12, container.count())
    }

    #[test]
    fn test_pawn_jumping_1() {
        let mut game = ChessGame::from_fen("4k3/4p3/8/8/8/8/4P3/4K3 w - - 0 1");
        game.block_castling();
        let mut container = SimpleMovesContainer::new();

        // In this position, white can move the pawn 2 squares up, but black can't
        // Count of valid positions: 8 for the knight, 4 for the king -> 12
        // white pawns: 2 -> 6
        // black pawns: 1 -> 13
        game.update_move_container(&mut container, true);
        assert_eq!(6, container.count());

        // Same for black
        game.update_move_container(&mut container, false);
        assert_eq!(6, container.count())
    }

    #[test]
    fn test_pawn_jumping_2() {
        let mut game = ChessGame::from_fen("8/8/4p3/8/8/4P3/8/8 w - - 0 1");
        game.block_castling();
        let mut container = SimpleMovesContainer::new();

        // In this pos, players have only 1 legal move

        game.update_move_container(&mut container, true);
        assert_eq!(1, container.count());

        // Same for black
        game.update_move_container(&mut container, false);
        assert_eq!(1, container.count())
    }


}
