use crate::model::moves::{Move, MoveQuality};

/// Stores a list of moves and retrieve them in an order that implementation can define
/// This allows to not have to sort a list of move based on an order.
pub trait MovesContainer {
    fn add(&mut self, m: Move);
    fn has_next(&self) -> bool;
    fn get_next(&mut self) -> Move;
}

pub struct SimpleMovesContainer {
    moves: Vec<Move>,
    index: usize,
}

impl SimpleMovesContainer {
    pub fn new() -> Self {
        Self { moves: Vec::with_capacity(64), index: 0 }
    }
}

impl MovesContainer for SimpleMovesContainer {
    fn add(&mut self, m: Move) {
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
}


pub struct SortedMovesContainer {
    /// The different containers
    containers: Vec<Vec<Move>>,
    /// index inside each container
    indices: Vec<usize>,
    /// The current container
    container_index: usize,
}

impl SortedMovesContainer {
    pub fn new() -> Self {
        Self {
            containers: vec![vec![], vec![]],
            indices: vec![0, 0],
            container_index: 0
        }
    }
}

impl MovesContainer for SortedMovesContainer {
    fn add(&mut self, m: Move) {
        match m.quality {
            MoveQuality::Capture => self.containers[0].push(m),
            MoveQuality::Motion => self.containers[1].push(m)
        }
    }

    fn has_next(&self) -> bool {
        // println!("{:?}", self.index < self.containers[self.container].len());
        // println!("{:?}", (self.container < self.containers.len()));
        // println!("{:?}", (self.containers[self.container + 1].len() > 0));

        false
    }

    fn get_next(&mut self) -> Move {
        let i = self.indices[self.container_index];
        if i < self.containers[self.container_index].len() {
            self.indices[self.container_index] += 1;
            self.containers[self.container_index][i]
        } else {
            self.container_index += 1;
            self.containers[self.container_index][0]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::moves::Move;
    use crate::model::moves_container::{MovesContainer, SortedMovesContainer};

    #[test]
    fn test_sorted_container() {
        let mut container = SortedMovesContainer::new();
        assert!(!container.has_next());

        let m1 = Move::new(0, 1, true);
        let m2 = Move::new(2, 3, true);
        let mut m3 = Move::new(4, 5, true);
        m3.set_as_capture();

        container.add(m1);
        container.add(m2);
        container.add(m3);

        assert!(container.has_next());

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

