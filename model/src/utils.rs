macro_rules! is_set {
    ($a: expr, $at: expr) => {
        (($a >> $at) & 1u64) == 1
    };
}

/// Set the bits
macro_rules! set_at {
    ($a:expr , $b:expr) => (
        $a |= 1u64 << $b
    )
}

macro_rules! clear_at {
    ($a:expr , $b:expr) => (
        $a &= !(1u64 << $b);
    )
}

/// A macro to iterate over the bits of an integer, processing each set bit.
///
/// # Parameters
/// - `$bits`: The integer value (e.g., u64) whose bits are to be processed.
/// - `$index`: A variable name that will be bound to the current set bit's index
///             during each iteration.
/// - `$body`: The code block to execute for each bit (inline, no closures involved).
///
macro_rules! consume_bits {
    ($bits:expr, $index:ident, $body:block) => {
        {
            let mut bits = $bits;
            while bits != 0 {
                // Get the position of the least significant set bit
                let $index = bits.trailing_zeros() as usize;
                // Inline the body code here
                $body
                // Clear the least significant set bit
                bits &= bits - 1;
            }
        }
    }
}

pub(crate) use is_set;
pub(crate) use set_at;
pub(crate) use clear_at;
pub(crate) use consume_bits;

pub type ChessPosition = i8;

/// Creates a `ChessPosition` from a rank (row) and file (column).
///
/// # Arguments
/// * `rank` - The rank (row) of the position, ranging from 0 to 7.
/// * `file` - The file (column) of the position, ranging from 0 to 7.
///
/// # Returns
/// * A valid `ChessPosition`, which is essentially an index ranging from 0 to 63.
///
/// # Panics
/// * The function panics if the rank or file is outside the valid range (0-7).
pub fn from_rank_file(rank: usize, file: usize) -> ChessPosition {
    assert!(rank < 8, "Rank must be between 0 and 7");
    assert!(file < 8, "File must be between 0 and 7");
    // Calculate the 1D index of the chessboard from rank and file
    (rank * 8 + file) as ChessPosition
}


pub trait IntoChessPosition {
    fn as_chess_position(&self) -> ChessPosition;
}

impl IntoChessPosition for &str {
    fn as_chess_position(&self) -> ChessPosition {
        chesspos_to_index(self).unwrap()
    }
}

// transforms a position (x,y) into a bit index
pub fn pos_to_index(x: ChessPosition, y: ChessPosition) -> ChessPosition {
    x + 8 * y
}

/// Convert an algebraic chess position to an integer
#[allow(dead_code)]
pub fn chesspos_to_index(text: &str) -> Option<ChessPosition> {
    let mut iter = text.chars();
    let first_char = iter.next()?;
    let second_char = iter.next()?;
    let row = second_char.to_digit(10).unwrap();
    let col = match first_char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Unknown chess position at char: {}", first_char)
    };
    Some(pos_to_index(col, row as ChessPosition - 1))
}

pub fn index_to_chesspos(index: ChessPosition) -> String {
    let x = index % 8;
    let y = index / 8 + 1;
    let s = match x {
        0 => "a".to_string(),
        1 => "b".to_string(),
        2 => "c".to_string(),
        3 => "d".to_string(),
        4 => "e".to_string(),
        5 => "f".to_string(),
        6 => "g".to_string(),
        7 => "h".to_string(),
        _ => panic!("Impossible position: {x}")
    };
    s + format!("{y}").as_str()
}

/// Prints all the bits of an integer as a grid
/// Used for debugging.
#[allow(dead_code)]
pub fn print_bitboard(bitboard: u64) {
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
