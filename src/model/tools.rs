macro_rules! is_set {
    ($a: expr, $at: expr) => {
        (($a >> $at) & 1u64) == 1
    };
}

/// Set the bits
macro_rules! set_at {
    ($a:expr , $b:expr) => (
        $a |= 1u64 << $b;
    )
}

macro_rules! clear_at {
    ($a:expr , $b:expr) => (
        $a &= !(1u64 << $b);
    )
}

pub(crate) use is_set;
pub(crate) use set_at;
pub(crate) use clear_at;

// transforms a position (x,y) into a bit index
pub fn pos_to_index(x: i8, y: i8) -> i8 {
    x + 8 * y
}

/// Convert an algreabraic chess position to an integer
pub fn chesspos_to_index(text: &str) -> i8 {
    let mut chars: Vec<char> = text.chars().collect();
    let row = chars[1].to_digit(10).unwrap();
    let col = match chars[0] {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Unknown chess position at char: {}", chars[0])
    };
    pos_to_index(col, row as i8 - 1)
}

pub fn index_to_chesspos(index: i8) -> String {
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
