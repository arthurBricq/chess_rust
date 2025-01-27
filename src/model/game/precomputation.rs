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

/// For each square and direction, precompute the "ray" of squares that would be attacked 
/// if no blocking pieces existed. It is up to the runtime to compute the actual attacked squares
/// using a mask.
/// 
/// Directions are defined as: N, S, E, W
fn sliding_attacks() -> ([u64; 64], [u64; 64], [u64; 64], [u64; 64]) {
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

    (north, south, east, west)
}


// TODO understand clearly if the way that I use these static variables it the right one
// I really must make sure that I am not using some sort of cloning.

pub static PAWN_ATTACK_MASKS: Lazy<([u64; 64], [u64; 64])> = Lazy::new(pawn_attacks);

pub static KNIGHT_ATTACK_MASKS: Lazy<[u64; 64]> = Lazy::new(knight_attacks);

pub static KING_ATTACK_MASKS: Lazy<[u64; 64]> = Lazy::new(king_attacks);

pub static SLIDING_ATTACK_MASKS: Lazy<([u64; 64], [u64; 64], [u64; 64], [u64; 64])> = Lazy::new(sliding_attacks);

