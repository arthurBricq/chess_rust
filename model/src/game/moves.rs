use crate::chess_type::Type::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::chess_type::Type;
use crate::game::attacks::ChessAttacks;
use crate::game::precomputation::{
    KING_ATTACK_MASKS, KNIGHT_ATTACK_MASKS, PAWN_ATTACK_MASKS, SLIDING_ATTACK_MASKS,
};
use crate::game::{ChessGame, FLAG_BLACK_KING_MOVED, FLAG_WHITE_KING_MOVED};
use crate::motion_iterator::StepMotionIterator;
use crate::moves::MoveQuality::{EqualCapture, GoodCapture};
use crate::moves::{
    Move, BLACK_PAWN_MOVES, KING_MOVES, KING_SPECIAL_MOVES, KNIGHT_MOVES, WHITE_PAWN_MOVES,
};
use crate::moves_container::MovesContainer;
use crate::utils::{consume_bits, is_set, pieces_for_color, ChessPosition};
use std::ops::Range;

impl ChessGame {
    /// Fills the provided container with all the available moves at the current position.
    ///
    /// This function also resets the move container before running anything.
    /// 
    /// TODO: remove this ! It's twice slower now :)
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
                        m.set_quality_from_scores(piece, captured);
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
                        // Note: all these casting don't have a performance impact.
                        // I have tested this manually and it's fine.
                        from as ChessPosition,
                        to as ChessPosition,
                        white_playing,
                    ));
                } else if is_set!(self.whites, to) != white_playing {
                    // TODO assert if it is worth checking which piece is being captures to have a better ordering
                    let mut m =
                        Move::new(from as ChessPosition, to as ChessPosition, white_playing);
                    if let Some(captured) = self.type_at_index(m.to) {
                        m.set_quality_from_scores(Knight, captured);
                    }
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
                    // A king can't possibly do a good capture...
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
                // For pawns to move on an attack, the square must be occupied, or be an en passant target
                let occupied = is_set!(occupancy, to);
                if occupied && (is_set!(self.whites, to) != white_playing) {
                    let mut m =
                        Move::new(from as ChessPosition, to as ChessPosition, white_playing);
                    // A pawn capture is considered good by default
                    m.set_quality(GoodCapture);
                    container.push(m);
                } else if !occupied && is_set!(self.en_passant_target, to) {
                    // En passant capture: diagonal to empty square matching ep target
                    let mut m =
                        Move::new(from as ChessPosition, to as ChessPosition, white_playing);
                    m.set_quality(GoodCapture);
                    container.push(m);
                }
            });
        });

        // 3.b Pawns motions

        let pieces = pieces_for_color!(self.whites, self.pawns, white_playing);
        consume_bits!(pieces, from, {
            if white_playing {
                if !is_set!(occupancy, from + 8) {
                    container.push(Move::new(
                        from as ChessPosition,
                        from as ChessPosition + 8,
                        white_playing,
                    ));
                }
            } else {
                if !is_set!(occupancy, from - 8) {
                    container.push(Move::new(
                        from as ChessPosition,
                        from as ChessPosition - 8,
                        white_playing,
                    ));
                }
            }

            // Pawn special moves: two squares up or down
            let rank = from / 8;
            if white_playing
                && rank == 1
                && !is_set!(occupancy, from + 8)
                && !is_set!(occupancy, from + 16)
            {
                container.push(Move::new(
                    from as ChessPosition,
                    from as ChessPosition + 16,
                    white_playing,
                ));
            }

            if !white_playing
                && rank == 6
                && !is_set!(occupancy, from - 8)
                && !is_set!(occupancy, from - 16)
            {
                container.push(Move::new(
                    from as ChessPosition,
                    from as ChessPosition - 16,
                    white_playing,
                ));
            }
        });

        // 4. Rooks

        let rook_left = pieces_for_color!(self.whites, self.rooks, white_playing);
        self.fill_attacked_squares_from_sliding_piece(
            rook_left,
            Rook,
            occupancy,
            0..4,
            white_playing,
            container,
        );

        // 5. Bishops

        let bishops = pieces_for_color!(self.whites, self.bishops, white_playing);
        self.fill_attacked_squares_from_sliding_piece(
            bishops,
            Bishop,
            occupancy,
            4..8,
            white_playing,
            container,
        );

        // 6. Queens

        let queens = pieces_for_color!(self.whites, self.queens, white_playing);
        self.fill_attacked_squares_from_sliding_piece(
            queens,
            // The score of the queen is the problem...
            Queen,
            occupancy,
            0..8,
            white_playing,
            container,
        );

        // 7. Castle

        // White castling

        if white_playing && !is_set!(self.flags, FLAG_WHITE_KING_MOVED) {
            let mut attacked: Option<u64> = None;

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

        // black castling

        if !white_playing && !is_set!(self.flags, FLAG_BLACK_KING_MOVED) {
            let mut attacked: Option<u64> = None;

            if !is_set!(occupancy, 61) && !is_set!(occupancy, 62) {
                if attacked.is_none() {
                    attacked = Some(self.get_attacked_squares(true));
                }
                if let Some(attacked) = attacked {
                    if !is_set!(attacked, 60) && !is_set!(attacked, 61) && !is_set!(attacked, 62) {
                        container.push(Move::new(60, 62, white_playing));
                    }
                }
            }

            // Check occupancy for black's large castle
            if !is_set!(occupancy, 59) && !is_set!(occupancy, 58) && !is_set!(occupancy, 57) {
                if attacked.is_none() {
                    attacked = Some(self.get_attacked_squares(true));
                }
                if let Some(attacked) = attacked {
                    if !is_set!(attacked, 60) && !is_set!(attacked, 59) && !is_set!(attacked, 58) {
                        container.push(Move::new(60, 58, white_playing));
                    }
                }
            }
        }
    }

    fn fill_attacked_squares_from_sliding_piece<T: MovesContainer>(
        &self,
        pieces: u64,
        t: Type,
        occupancy: u64,
        direction_indices: Range<usize>,
        white_playing: bool,
        container: &mut T,
    ) {
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
                        if let Some(captured) = self.type_at_index(m.to) {
                            m.set_quality_from_scores(t, captured);
                        }
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
    use crate::moves::Move;
    use crate::moves::MoveQuality::{EqualCapture, GoodCapture, LowCapture, Motion};
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

    fn assert_container_contains(container: &mut dyn MovesContainer, expected: Move) -> bool {
        while container.has_next() {
            let next = container.pop_next_move();
            if next == expected {
                return true;
            }
        }
        false
    }

    #[test]
    fn test_small_castle_no_enemies() {
        let game = ChessGame::from_fen("4k2r/4pppp/8/8/8/8/4PPPP/4K2R w - - 0 1");
        let mut container = SimpleMovesContainer::new();

        // White can small castle
        game.update_move_container(&mut container, true);
        assert!(assert_container_contains(
            &mut container,
            Move::new(4, 6, true)
        ));

        // Black can small castle
        game.update_move_container(&mut container, false);
        assert!(assert_container_contains(
            &mut container,
            Move::new(60, 62, false)
        ))
    }

    /// Asserts that every move on the move container matches 1 and exactly one matcher.
    /// The order of the matcher does not matter.
    fn asserts_moves_matching(
        container: &mut dyn MovesContainer,
        mut matchers: Vec<fn(&Move) -> bool>,
    ) {
        while container.has_next() {
            let next = container.pop_next_move();
            let matched_predicate = matchers.iter().position(|f| f(&next));
            if let Some(index) = matched_predicate {
                matchers.remove(index);
            } else {
                panic!("Unexpected move for which no predicate worked: {:?}", next);
            }
        }
    }

    #[test]
    fn test_move_evaluation_1() {
        let mut game = ChessGame::from_fen("8/8/8/8/8/1n6/2p5/N7 w - - 0 1");
        game.block_castling();
        let mut container = SimpleMovesContainer::new();

        // White has two moves: 1 equal capture and one lower capture
        game.update_move_container(&mut container, true);

        asserts_moves_matching(
            &mut container,
            vec![|m: &Move| m.quality == EqualCapture, |m: &Move| {
                m.quality == LowCapture
            }],
        )
    }

    #[test]
    fn test_move_evaluation_2() {
        let mut game = ChessGame::from_fen("8/8/8/8/8/1q6/8/N7 w - - 0 1");
        game.block_castling();
        let mut container = SimpleMovesContainer::new();

        // White has two moves: 1 equal capture and one lower capture
        game.update_move_container(&mut container, true);

        asserts_moves_matching(
            &mut container,
            vec![|m: &Move| m.quality == Motion, |m: &Move| {
                m.quality == GoodCapture
            }],
        )
    }

    #[test]
    fn test_move_evaluation_3() {
        let mut game = ChessGame::from_fen("8/8/3p1b2/4B3/3q1P2/8/8/8 w - - 0 1");
        game.block_castling();
        let mut container = SimpleMovesContainer::new();

        // White has two moves: 1 equal capture and one lower capture
        game.update_move_container(&mut container, true);

        asserts_moves_matching(
            &mut container,
            vec![
                |m: &Move| m.quality == LowCapture,
                |m: &Move| m.quality == EqualCapture,
                |m: &Move| m.quality == GoodCapture,
                |m: &Move| m.quality == Motion,
            ],
        )
    }
}
