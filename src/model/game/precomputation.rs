use crate::model::utils::{from_rank_file, ChessPosition};
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
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
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
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
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

/// For each square and direction, computes the "ray" of squares that would be attacked.
///
/// Each position of the board is associated with a vector of positions, as the ray expand in the
/// given direction.
///
/// Directions are defined as: N, S, E, W, as per the ordering defined in the class.
fn sliding_attacks() -> [[Vec<ChessPosition>; 64]; 8] {
    // Use std::array::from_fn to initialize the 2D array (1st dimension is the direction)
    std::array::from_fn(|direction_index| {
        // For each square on the board, compute the sliding ray for the given direction
        std::array::from_fn(|sq| {
            let mut ray = Vec::new();
            let rank = sq / 8;
            let file = sq % 8;

            match direction_index {
                0 => {
                    // North (upwards direction)
                    for r in rank + 1..8 {
                        ray.push(from_rank_file(r, file));
                    }
                }
                1 => {
                    // South (downwards direction)
                    for r in (0..rank).rev() {
                        ray.push(from_rank_file(r, file));
                    }
                }
                2 => {
                    // East (right direction)
                    for f in file + 1..8 {
                        ray.push(from_rank_file(rank, f));
                    }
                }
                3 => {
                    // West (left direction)
                    for f in (0..file).rev() {
                        ray.push(from_rank_file(rank, f));
                    }
                }
                4 => {
                    // Northeast (up-right)
                    let mut r = rank as isize + 1;
                    let mut f = file as isize + 1;
                    while r < 8 && f < 8 {
                        ray.push(from_rank_file(r as usize, f as usize));
                        r += 1;
                        f += 1;
                    }
                }
                5 => {
                    // Northwest (up-left)
                    let mut r = rank as isize + 1;
                    let mut f = file as isize - 1;
                    while r < 8 && f >= 0 {
                        ray.push(from_rank_file(r as usize, f as usize));
                        r += 1;
                        f -= 1;
                    }
                }
                6 => {
                    // Southeast (down-right)
                    let mut r = rank as isize - 1;
                    let mut f = file as isize + 1;
                    while r >= 0 && f < 8 {
                        ray.push(from_rank_file(r as usize, f as usize));
                        r -= 1;
                        f += 1;
                    }
                }
                7 => {
                    // Southwest (down-left)
                    let mut r = rank as isize - 1;
                    let mut f = file as isize - 1;
                    while r >= 0 && f >= 0 {
                        ray.push(from_rank_file(r as usize, f as usize));
                        r -= 1;
                        f -= 1;
                    }
                }
                _ => unreachable!(),
            }

            ray
        })
    })
}

// TODO understand clearly if the way that I use these static variables it the right one
// I really must make sure that I am not using some sort of cloning.

pub static PAWN_ATTACK_MASKS: Lazy<([u64; 64], [u64; 64])> = Lazy::new(pawn_attacks);

pub static KNIGHT_ATTACK_MASKS: Lazy<[u64; 64]> = Lazy::new(knight_attacks);

pub static KING_ATTACK_MASKS: Lazy<[u64; 64]> = Lazy::new(king_attacks);

pub static SLIDING_ATTACK_MASKS: Lazy<[[Vec<ChessPosition>; 64]; 8]> = Lazy::new(sliding_attacks);
