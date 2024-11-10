use crate::model::moves::{Move, MoveQuality};

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
}

/// A move container which
/// * keeps move in a sorted datastructures, so that good moves are retrieved before others.
/// * allows to store a "first move", typically obtained from iterative deepening, which is retrieved
///   before all the moves in the containers.
pub struct SmartMoveContainer {
    /// If some, always returns this move before the others
    /// Once this move is returned, it is immediately consumed
    first_move: Option<Move>,
    /// The different containers.
    /// We use arrays and not vectors to be more efficient
    containers: [[Move; 128]; 4],
    /// size of each containers
    lens: [usize; 4],
    /// index inside each container
    indices: [usize; 4],
}

impl SmartMoveContainer {
    pub fn new() -> Self {
        Self {
            first_move: None,
            containers: [[Move::new(0, 0, true); 128]; 4],
            lens: [0; 4],
            indices: [0; 4],
        }
    }
}

impl MovesContainer for SmartMoveContainer {
    fn push(&mut self, m: Move) {
        let index = match m.quality {
            MoveQuality::GoodCapture => 0,
            MoveQuality::EqualCapture => 1,
            MoveQuality::LowCapture => 2,
            MoveQuality::Motion => 3,
        };
        self.containers[index][self.lens[index]] = m;
        self.lens[index] += 1;
    }

    fn has_next(&self) -> bool {
        self.indices[0] < self.lens[0]
            || self.indices[1] < self.lens[1]
            || self.indices[2] < self.lens[2]
            || self.indices[3] < self.lens[3]
            || self.first_move.is_some()
    }

    // TODO Maybe using VecDeques could make this implementation faster !
    fn get_next(&mut self) -> Move {
        // If there is a first move stored, consume it.
        if self.first_move.is_some() {
            return self.first_move.take().unwrap();
        }
        // Otherwise, consume the different lists.
        let index = if self.indices[0] < self.lens[0] {
            0
        } else if self.indices[1] < self.lens[1] {
            1
        } else if self.indices[2] < self.lens[2] {
            2
        } else {
            3
        };
        self.indices[index] += 1;
        self.containers[index][self.indices[index] - 1]
    }

    fn reset(&mut self) {
        self.lens = [0; 4];
        self.indices = [0; 4];
        self.first_move = None;
    }

    fn count(&self) -> usize {
        self.lens.iter().sum()
    }

    fn set_first_move(&mut self, m: Move) {
        // TODO Maybe removing the move from the existing container is a good thing to do.
        self.first_move = Some(m)
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

