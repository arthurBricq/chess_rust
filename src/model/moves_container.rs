use crate::model::moves::{Move, MoveQuality};

/// Stores a list of moves and retrieve them in an order that implementation can define
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
    containers: Vec<Vec<Move>>,
    index: usize,
    container: usize,
}

impl SortedMovesContainer {
    pub fn new() -> Self {
        Self { containers: vec![vec![], vec![]], index: 0, container: 0 }
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
        println!("{}, {}", self.index, self.container);
        println!("{:?}", self.containers);
        // println!("{:?}", self.index < self.containers[self.container].len());
        // println!("{:?}", (self.container < self.containers.len()));
        // println!("{:?}", (self.containers[self.container + 1].len() > 0));
        // TODO this will not work with three category


        self.index < self.containers[self.container].len() || (self.container < self.containers.len() && self.containers[self.container + 1].len() > 0)
    }

    fn get_next(&mut self) -> Move {
        let i = self.index;
        self.index += 1;
        if i < self.containers[self.container].len() {
            self.containers[self.container][i]
        } else {
            self.container += 1;
            self.index = 1;
            self.containers[self.container][0]
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

