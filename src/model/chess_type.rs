#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

pub type ScoreType = i64;

impl Type {
    pub fn score(&self) -> ScoreType {
        match self {
            Type::Pawn => 1,
            Type::Bishop => 3,
            Type::Knight => 3,
            Type::Rook => 5,
            Type::Queen => 9,
            Type::King => 10000
        }
    }
}
