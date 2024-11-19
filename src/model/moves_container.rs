use crate::model::moves::Move;
use crate::model::moves::MoveQuality::{KillerMove, Principal};
use std::collections::BinaryHeap;

/// Stores a list of moves and retrieve them in an order that implementation can define
/// This allows to not have to sort a list of move based on an order.
pub trait MovesContainer {
    fn push(&mut self, m: Move);
    fn has_next(&self) -> bool;
    fn get_next(&mut self) -> Move;
    fn reset(&mut self);
    fn count(&self) -> usize;
    /// Asks to retain the given move as the first move to evaluate
    fn set_first_move(&mut self, m: Move);
    /// Add killer move
    /// A killer is a move that produced a cutoff at the same depth
    fn add_killer_move(&mut self, m: Move);
}

pub struct SimpleMovesContainer {
    pub moves: Vec<Move>,
    index: usize,
}

impl SimpleMovesContainer {
    pub fn new() -> Self {
        Self { moves: Vec::with_capacity(128), index: 0 }
    }
}

impl MovesContainer for SimpleMovesContainer {
    fn push(&mut self, m: Move) {
        self.moves.push(m)
    }

    fn has_next(&self) -> bool {
        self.index < self.moves.len()
    }

    fn get_next(&mut self) -> Move {
        let i = self.index;
        self.index += 1;
        self.moves[i]
    }

    fn reset(&mut self) {
        self.moves.clear();
        self.index = 0;
    }

    fn count(&self) -> usize {
        self.moves.len()
    }

    fn set_first_move(&mut self, _m: Move) {
        todo!()
    }

    fn add_killer_move(&mut self, _m: Move) {
        todo!()
    }
}

/// A move container which
/// * keeps move in a sorted datastructures, so that good moves are retrieved before others.
/// * allows to store a "first move", typically obtained from iterative deepening, which is retrieved
///   before all the moves in the containers.
pub struct SmartMoveContainer {
    moves: BinaryHeap<Move>,
}

impl SmartMoveContainer {
    pub fn new() -> Self {
        Self {
            moves: BinaryHeap::with_capacity(128)
        }
    }
}

impl MovesContainer for SmartMoveContainer {
    fn push(&mut self, m: Move) {
        self.moves.push(m)
    }

    fn has_next(&self) -> bool {
        self.moves.len() > 0
    }

    fn get_next(&mut self) -> Move {
        self.moves.pop().unwrap()
    }

    fn reset(&mut self) {
        self.moves.clear();
    }

    fn count(&self) -> usize {
        self.moves.len()
    }

    fn set_first_move(&mut self, mut m: Move) {
        // TODO Maybe removing the move from the existing container is a good thing to do.
        m.set_quality(Principal);
        self.moves.push(m);
    }

    fn add_killer_move(&mut self, mut m: Move) {
        // TODO Maybe removing the move from the existing container is a good thing to do.
        m.set_quality(KillerMove);
        self.moves.push(m);
    }
}

#[cfg(test)]
mod tests {
    use crate::model::moves::Move;
    use crate::model::moves::MoveQuality::GoodCapture;
    use crate::model::moves_container::{MovesContainer, SmartMoveContainer};

    #[test]
    fn test_sorted_container() {
        let mut container = SmartMoveContainer::new();
        assert!(!container.has_next());

        let m1 = Move::new(0, 1, true);
        let m2 = Move::new(2, 3, true);
        let mut m3 = Move::new(4, 5, true);
        m3.set_quality(GoodCapture);

        container.push(m1);
        container.push(m2);
        container.push(m3);

        assert!(container.has_next());
        assert_eq!(3, container.count());

        // The first value (m3) is supposed to be a capture
        assert!(container.get_next().is_capture());
        assert!(container.has_next());

        // the next two values are not capture move
        assert!(!container.get_next().is_capture());
        assert!(container.has_next());

        assert!(!container.get_next().is_capture());

        // now the container is empty
        assert!(!container.has_next());
    }

    #[test]
    fn test_first_move() {
        let mut container = SmartMoveContainer::new();
        let m1 = Move::new(0, 1, true);
        let m2 = Move::new(2, 3, true);
        let mut m3 = Move::new(4, 5, true);
        m3.set_quality(GoodCapture);
        container.push(m2);
        container.push(m3);
        container.set_first_move(m1);

        let first = container.get_next();
        let second = container.get_next();
        let third = container.get_next();

        assert_eq!(first, m1);
        assert_eq!(second, m3);
        assert_eq!(third, m2);
    }
}

