use crate::model::moves::{Move, MoveQuality};

/// Stores a list of moves and retrieve them in an order that implementation can define
/// This allows to not have to sort a list of move based on an order.
pub trait MovesContainer {
    fn push(&mut self, m: Move);
    fn has_next(&self) -> bool;
    fn get_next(&mut self) -> Move;
    fn reset(&mut self);
    fn count(&self) -> usize;
}

pub struct SimpleMovesContainer {
    pub moves: Vec<Move>,
    index: usize,
}

impl SimpleMovesContainer {
    pub fn new() -> Self {
        Self { moves: Vec::with_capacity(64), index: 0 }
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
}


pub struct SortedMovesContainer {
    /// The different containers.
    /// We use arrays and not vectors to be more efficient
    containers: [[Move; 128]; 4],
    /// size of each containers
    lens: [usize; 4],
    /// index inside each container
    indices: [usize; 4],
}

impl SortedMovesContainer {
    pub fn new() -> Self {
        Self {
            containers: [[Move::new(0, 0, true); 128]; 4],
            lens: [0; 4],
            indices: [0; 4],
        }
    }
}

impl MovesContainer for SortedMovesContainer {
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
    }

    fn get_next(&mut self) -> Move {
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
    }

    fn count(&self) -> usize {
        self.lens.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::moves::Move;
    use crate::model::moves::MoveQuality::GoodCapture;
    use crate::model::moves_container::{MovesContainer, SortedMovesContainer};

    #[test]
    fn test_sorted_container() {
        let mut container = SortedMovesContainer::new();
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
}

