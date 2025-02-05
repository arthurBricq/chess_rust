use crate::moves::Move;
use crate::moves::MoveQuality::{KillerMove, Principal};
use std::collections::BinaryHeap;

/// Stores a list of moves and retrieve them in an order that implementation can define
/// This allows to not have to sort a list of move based on an order.
pub trait MovesContainer {
    /// Adds a new move in the container
    fn push(&mut self, m: Move);
    /// Returns true if the container has some moves
    fn has_next(&self) -> bool;
    /// Consumes one move of the container
    fn pop_next_move(&mut self) -> Move;
    /// Clear all moves in the container
    fn reset(&mut self);
    /// Returns the number of moves in the container
    fn count(&self) -> usize;
    /// Asks to retain the given move as the first move to evaluate
    fn set_first_move(&mut self, m: Move);
    /// Add killer move
    /// A killer is a move that produced a cutoff at the same depth
    fn add_killer_move(&mut self, m: Move);
    /// Function that prints all the moves in the container
    /// This function removes all the moves from the container
    /// Note: if you just need to print,
    fn debug(&mut self) {
        while self.has_next() {
            let m = self.pop_next_move();
            println!("{m}");
        }
    }
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

    fn pop_next_move(&mut self) -> Move {
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

    fn pop_next_move(&mut self) -> Move {
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
    use crate::chess_type::Type::Pawn;
    use crate::game::ChessGame;
    use crate::moves::Move;
    use crate::moves::MoveQuality::GoodCapture;
    use crate::moves_container::{MovesContainer, SmartMoveContainer};

    #[test]
    fn test_moves_container_with_basic_position() {
        let mut game = ChessGame::empty();
        game.block_castling();
        game.set_piece(Pawn, true, "e2");
        game.set_piece(Pawn, false, "e7");

        let mut container = SmartMoveContainer::new();
        game.update_move_container(&mut container, true);
        let count = container.count();
        container.debug();
        assert_eq!(2, count);

        game.update_move_container(&mut container, false);
        let count = container.count();
        container.debug();
        assert_eq!(2, count);
    }

    #[test]
    fn test_moves_container_with_standard_position() {
        let game = ChessGame::standard_game();

        let mut container = SmartMoveContainer::new();

        // there are twenty possible positions
        game.update_move_container(&mut container, true);
        assert_eq!(20, container.count());

        game.update_move_container(&mut container, false);
        assert_eq!(20, container.count());
    }

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
        assert!(container.pop_next_move().is_capture());
        assert!(container.has_next());

        // the next two values are not capture move
        assert!(!container.pop_next_move().is_capture());
        assert!(container.has_next());

        assert!(!container.pop_next_move().is_capture());

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

        let first = container.pop_next_move();
        let second = container.pop_next_move();
        let third = container.pop_next_move();

        assert_eq!(first, m1);
        assert_eq!(second, m3);
        assert_eq!(third, m2);
    }

    #[test]
    fn test_possible_moves_with_fen_game() {
        let fen = "6r1/p1q3bk/4rnR1/2p2Q1P/1p1p4/3P2P1/2PK1B2/8 w - - 0 46";
        let game = ChessGame::from_fen(fen);
        let mut container = SmartMoveContainer::new();
        game.update_move_container(&mut container, true);
        while container.has_next() {
            let m = container.pop_next_move();
            assert_ne!(m.from, m.to);
            assert_eq!(true, m.is_white);
            println!("{m}")
        }
    }
}

