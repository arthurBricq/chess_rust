use once_cell::sync::Lazy;

/// Computes the attack masks for pawns
/// 
/// For each position of the chess board, one attack mask is an `u64` where the ones indicate that
/// there is an attack.
/// 
/// This function returns the attack mask for white and black pawns.
fn computes_pawn_attacks() -> ([u64; 64], [u64; 64]) {
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

pub static PAWN_ATTACK_MASKS: Lazy<([u64; 64], [u64; 64])> = Lazy::new(computes_pawn_attacks);
