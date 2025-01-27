use once_cell::sync::Lazy;

/// Computes the attack masks for pawns
/// 
/// For each position of the chess board, one attack mask is an `u64` where the ones indicate that
/// there is an attack.
/// 
/// This function returns the attack mask for white and black pawns.
fn pawn_attacks() -> ([u64; 64], [u64; 64]) {
    let mut white_attacks = [0u64; 64];
    let mut black_attacks = [0u64; 64];

    for position in 0..64 {
        let mut mask = 0;
        let rank = position / 8;
        let file = position % 8;

        // White pawn attacks
        if rank < 7 {
            if file > 0 {
                mask |= 1 << (position + 7);
            } // Attack left
            if file < 7 {
                mask |= 1 << (position + 9);
            } // Attack right
        }
        white_attacks[position] = mask;

        mask = 0;
        // Black pawn attacks
        if rank > 0 {
            if file > 0 {
                mask |= 1 << (position - 9);
            } // Attack left
            if file < 7 {
                mask |= 1 << (position - 7);
            } // Attack right
        }
        black_attacks[position] = mask;
    }

    (white_attacks, black_attacks)
}

fn knight_attacks() -> [u64; 64] {
    let mut attacks = [0u64; 64];

    for sq in 0..64 {
        let mut mask = 0;
        let rank = sq / 8;
        let file = sq % 8;

        let moves = [
            (2, 1), (2, -1), (-2, 1), (-2, -1),
            (1, 2), (1, -2), (-1, 2), (-1, -2),
        ];

        for (dr, df) in moves {
            let r = rank as isize + dr;
            let f = file as isize + df;
            if r >= 0 && r < 8 && f >= 0 && f < 8 {
                mask |= 1 << (r * 8 + f);
            }
        }

        attacks[sq] = mask;
    }

    attacks
}


fn king_attacks() -> [u64; 64] {
    let mut attacks = [0u64; 64];

    for sq in 0..64 {
        let mut mask = 0;
        let rank = sq / 8;
        let file = sq % 8;

        let moves = [
            (0, 1), (1, 1), (1, 0), (1, -1),
            (0, -1), (-1, -1), (-1, 0), (-1, 1),
        ];

        for (dr, df) in moves {
            let r = rank as isize + dr;
            let f = file as isize + df;
            if r >= 0 && r < 8 && f >= 0 && f < 8 {
                mask |= 1 << (r * 8 + f);
            }
        }

        attacks[sq] = mask;
    }

    attacks
}

/// Directions that can be used for a sliding attack
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn to_index(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
        }
    }

    pub fn ray_mask(&self) -> [u64; 64] {
        // TODO this might be very inneficient, I can optimize later...
        SLIDING_ATTACK_MASKS[self.to_index()]
    }

    pub fn ray_attacks(&self, from: usize) -> u64 {
        self.ray_mask()[from]
    }

    pub fn compute_closest_blocker(&self, blockers: u64) -> u64 {
        match self {
            Direction::North | Direction::East => blockers & !(blockers.wrapping_sub(1)),
            Direction::South | Direction::West => blockers & blockers.wrapping_sub(1)
        }
    }

}

/// For each square and direction, precompute the "ray" of squares that would be attacked 
/// if no blocking pieces existed. It is up to the runtime to compute the actual attacked squares
/// using a mask.
/// 
/// Directions are defined as: N, S, E, W
fn sliding_attacks() -> [[u64; 64]; 4] {
    let mut north = [0u64; 64];
    let mut south = [0u64; 64];
    let mut east = [0u64; 64];
    let mut west = [0u64; 64];

    for sq in 0..64 {
        let rank = sq / 8;
        let file = sq % 8;

        // North
        for r in (rank + 1)..8 {
            north[sq] |= 1 << (r * 8 + file);
        }
        // South
        for r in (0..rank).rev() {
            south[sq] |= 1 << (r * 8 + file);
        }
        // East
        for f in (file + 1)..8 {
            east[sq] |= 1 << (rank * 8 + f);
        }
        // West
        for f in (0..file).rev() {
            west[sq] |= 1 << (rank * 8 + f);
        }
    }

    // Make sure to use the correct ordering !
    [north, south, east, west]
}


// TODO understand clearly if the way that I use these static variables it the right one
// I really must make sure that I am not using some sort of cloning.

pub static PAWN_ATTACK_MASKS: Lazy<([u64; 64], [u64; 64])> = Lazy::new(pawn_attacks);

pub static KNIGHT_ATTACK_MASKS: Lazy<[u64; 64]> = Lazy::new(knight_attacks);

pub static KING_ATTACK_MASKS: Lazy<[u64; 64]> = Lazy::new(king_attacks);

pub static SLIDING_ATTACK_MASKS: Lazy<[[u64; 64]; 4]> = Lazy::new(sliding_attacks);

mod tests {
    use crate::model::chess_type::Type::{Pawn, Rook};
    use crate::model::game::ChessGame;
    use crate::model::game::precomputation::Direction::{North, South};
    use crate::model::game::precomputation::{Direction, SLIDING_ATTACK_MASKS};
    use crate::model::game_constructor::GameConstructor;
    use crate::model::utils::IntoChessPosition;

    /// Prints all the bits of an integer as a grid
    /// Used for debugging.
    fn print_bitboard(bitboard: u64) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = rank * 8 + file;
                let bit = (bitboard >> square) & 1;
                print!("{} ", bit);
            }
            println!();
        }
        println!();
    }

    /// Test the way to find the "closest blocker"
    /// This is not really a unit-test, it's more of a way to get some code to execute and test it.
    #[test]
    fn test_finding_closest_blocker() {
        let mut chess_game = GameConstructor::empty();
        chess_game.set_piece(Pawn, true, "a3");
        chess_game.set_piece(Pawn, true, "a2");
        chess_game.set_piece(Pawn, true, "a4");
        chess_game.set_piece(Pawn, true, "a6");
        chess_game.set_piece(Pawn, true, "d1");
        chess_game.set_piece(Pawn, true, "f1");

        fn assert_correct_blocker(mut chess_game: ChessGame, ray_direction: Direction, rook_pos: &str, expected_blocker_pos: &str) {
            chess_game.set_piece(Rook, true, rook_pos);
            let rooks = chess_game.rooks;
            let pieces = chess_game.pawns;
            let sq = rooks.trailing_zeros() as usize;

            let ray = ray_direction.ray_attacks(sq);
            let blockers = ray & pieces;

            println!("Pieces: ");
            print_bitboard(pieces);

            println!("Blocker:");
            print_bitboard(blockers);

            // Finding the closest blocker
            // Problem is, depending on the direction, there are different possible formula...
            // My current take on the problem is that chatGPT might be simply wrong.
            // I should read carefully the descriptions, and decide if I pursue this discussion.
            // Otherwise, I could generate
            // - for every position
            // - for every direction
            // 1 vector of "next" positions
            // Then, finding the closest hit is very simple...

            // let closest_blocker = blockers & blockers.wrapping_sub(1);
            // let closest_blocker = blockers & !(blockers.wrapping_sub(1));
            let closest_blocker = ray_direction.compute_closest_blocker(blockers);

            let blocker_pos = closest_blocker.trailing_zeros() as usize;

            assert_eq!(1, closest_blocker.count_ones(), "There should be only one blocker");
            assert_eq!(expected_blocker_pos.into_position() as usize, blocker_pos);
        }

        // assert_correct_blocker(chess_game.clone(), North, "a1", "a3");
        // assert_correct_blocker(chess_game.clone(), North, "a5", "a6");

        // assert_correct_blocker(chess_game.clone(), South, "a8", "a6");
        assert_correct_blocker(chess_game.clone(), South, "a5", "a3");

    }
}